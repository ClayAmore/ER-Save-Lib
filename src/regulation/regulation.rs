use super::params::{param::PARAM, params::Params};
use crate::{
    param_trait::Param,
    regulation::{
        bnd4::bnd4::{BND4ParseError, BND4},
        dcx_zstd::dcx_zstd::DCXZSTD,
    },
};
use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use deku::{
    ctx::Endian, reader::Reader, writer::Writer, DekuContainerRead, DekuError, DekuReader,
    DekuWriter,
};
use std::{borrow::Cow, collections::HashMap, fs, io::Cursor, path::Path, sync::OnceLock};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegulationParseError {
    #[error(transparent)]
    DekuError(#[from] DekuError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    BND4ParseError(#[from] BND4ParseError),
    #[error("Failed to decrypt regulation file!")]
    DecryptionError,
    #[error("Param {} not found", .0)]
    ParamNotFound(&'static str),
}

// Context params for Regulation
type Ctx = (Endian, RegulationIdentifier);

// Regulation size can be determined from the regulation version. This
// needs to be done when reading regulation from the save file. However when
// reading regulation from a file the size is already known.
pub(crate) enum RegulationIdentifier {
    Version(u32),
    Size(usize),
}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct Regulation {
    raw: Vec<u8>,
    pub(crate) content: DCXZSTD<BND4<Params>>,
}

// DekuReader
impl<'a> DekuReader<'a, Ctx> for Regulation {
    fn from_reader_with_ctx<R: std::io::Read>(
        reader: &mut Reader<R>,
        ctx: Ctx,
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        // Grab regulation_identifier from the context params
        let (_, regulation_identifier) = ctx;

        // If the identifer is size then just store it, if it's version then
        // look up the size from the version to size map.
        let size = match regulation_identifier {
            RegulationIdentifier::Version(version) => {
                if let Some(size) = Self::ver_size_map().get(&version) {
                    size.to_owned()
                } else {
                    return Err(DekuError::Parse(Cow::from(format!(
                        "Failed to fetch regulation size from version: {}",
                        version
                    ))));
                }
            }
            RegulationIdentifier::Size(size) => size,
        };

        // Prepare a buffer to read the regulation bytes.
        let mut bytes = vec![0; size];

        // Read the regulation bytes.
        let _ = reader.read_bytes(size, &mut bytes)?;

        // Store a copy of the bytes to use when write. This is due
        // to the zstd compression configuartion being unknown. Because of that
        // writing edited regulation to the save file is not possible.
        let raw = bytes.clone();

        // Try to decrypt the regulation bytes
        let result = Self::decrypt(&bytes);

        // Throw error if decryption failed
        if let Err(err) = result {
            return Err(DekuError::Parse(Cow::from(format!(
                "Failed to decrypt regulation! {err})"
            ))));
        }

        // Unwrap decryption result
        let mut bytes = result.unwrap();

        // Try to parse the regulation
        let result = DCXZSTD::<BND4<Params>>::from_bytes((&mut bytes, 0));

        // Throw error if regulation parsing failed
        if let Err(err) = result {
            return Err(DekuError::Parse(Cow::from(format!(
                "Failed to parse dcx! {err})"
            ))));
        }

        // Unwrap regulation result
        let (_, dcx_zstd) = result.unwrap();

        // Store a raw copy of the bytes along with the parsed regulation
        Ok(Regulation {
            raw,
            content: dcx_zstd,
        })
    }
}

// DekuWriter
impl DekuWriter<Ctx> for Regulation {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut Writer<W>,
        _: Ctx,
    ) -> Result<(), DekuError> {
        // Due to the zstd compression configuration not being known, it's not
        // possible to compress the regulation back to how it originally was in the save
        // file. So we write back the same regulation bytes we read.
        self.raw.to_writer(writer, ())?;
        Ok(())
    }
}

impl Regulation {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, RegulationParseError> {
        let mut bytes = fs::read(path)?;
        Self::from_slice(&mut bytes)
    }

    pub fn from_slice(bytes: &mut [u8]) -> Result<Self, RegulationParseError> {
        let size = bytes.len();
        let mut cursor = Cursor::new(bytes);
        let mut reader = Reader::new(&mut cursor);
        Ok(Regulation::from_reader_with_ctx(
            &mut reader,
            (Endian::Little, RegulationIdentifier::Size(size)),
        )?)
    }

    fn decrypt(bytes: &[u8]) -> Result<Vec<u8>, RegulationParseError> {
        type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
        let key = [
            0x99, 0xBF, 0xFC, 0x36, 0x6A, 0x6B, 0xC8, 0xC6, 0xF5, 0x82, 0x7D, 0x09, 0x36, 0x02,
            0xD6, 0x76, 0xC4, 0x28, 0x92, 0xA0, 0x1C, 0x20, 0x7F, 0xB0, 0x24, 0xD3, 0xAF, 0x4E,
            0x49, 0x3F, 0xEF, 0x99,
        ];
        let iv = &bytes[0..16];
        let mut buf = bytes[16..bytes.len()].to_vec();
        if let Ok(pt) =
            Aes256CbcDec::new(&key.into(), iv.into()).decrypt_padded_mut::<NoPadding>(&mut buf)
        {
            Ok(pt.to_owned())
        } else {
            Err(RegulationParseError::DecryptionError)
        }
    }

    pub(crate) fn get_param<P: Param>(
        &self,
    ) -> Result<HashMap<i32, P::ParamType>, RegulationParseError> {
        let version = self.content.data.header.version;
        if let Some(param_bytes) = self.content.data.file_data.param_files.get(P::PARAM_NAME) {
            let mut cursor = Cursor::new(&param_bytes);
            let mut reader = Reader::new(&mut cursor);
            let l_param = PARAM::<P>::from_reader_with_ctx(&mut reader, version)?;

            let rows: HashMap<i32, P::ParamType> = l_param
                .row_headers
                .iter()
                .zip(l_param.row_data)
                .map(|(header, data)| (header.id, data))
                .collect();

            return Ok(rows);
        }
        Err(RegulationParseError::ParamNotFound(P::PARAM_NAME))
    }

    pub(crate) fn ver_size_map() -> &'static HashMap<u32, usize> {
        static VER_SIZE_MAP: OnceLock<HashMap<u32, usize>> = OnceLock::new();
        VER_SIZE_MAP.get_or_init(|| {
            HashMap::from([
                (10210038, 1811088),
                (10310059, 1829216),
                (10320064, 1829232),
                (10330078, 1829232),
                (10410090, 1831120),
                (10420097, 1831120),
                (10501000, 1831600),
                (10601000, 1834080),
                (10701000, 1838496),
                (10710188, 1838496),
                (10801000, 1844112),
                (10811000, 1844112),
                (10901000, 1852464),
                (10911000, 1852608),
                (11001000, 1859440),
                (11210015, 2006960),
                (11220021, 2006960),
                (11240023, 2001488),
                (11310027, 2025152),
                (11320031, 2019824),
                (11410033, 2036288),
                (11501000, 2036272),
                (11601000, 2036272),
            ])
        })
    }
}
