use super::{buckets::Buckets, file_header::BND4FileHeader, header::BND4Header};
use deku::prelude::*;
use deku::{DekuError, DekuRead, DekuWrite};
use encoding_rs::SHIFT_JIS;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BND4ParseError {
    #[error(transparent)]
    DekuError(#[from] DekuError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

// Context params for BND4Hheader
type Ctx<'a> = (&'a BND4Header, &'a Vec<BND4FileHeader>, &'a Vec<String>);

/// A struct representing Elden Ring BND4 file
///
/// This struct provides methods to read an Elden Ring BND4 file fomr a specified file path
/// or from a byte slice, and to check if a byte slice can be parsed as a valid BND4 file.
/// It includes functions for reading, parsing, and validating BND4 files, ensuring that
/// implementations handle the respective file formats correctly.
///
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
pub(crate) struct BND4<T: for<'a> DekuWriter<Ctx<'a>> + for<'a> DekuReader<'a, Ctx<'a>>> {
    pub(crate) header: BND4Header,
    #[deku(
        count = "header.file_count",
        ctx = "header.format, header.bit_big_endian"
    )]
    pub(crate) file_headers: Vec<BND4FileHeader>,
    #[deku(
        reader = "BND4::<T>::read_file_names(deku::reader, header.unicode, header.file_count)",
        writer = "BND4::<T>::write_file_names(deku::writer, header.unicode, file_names)"
    )]
    pub(crate) file_names: Vec<String>,
    #[deku(assert_eq = "0")]
    zero: u16,
    #[deku(ctx = "header.file_count")]
    buckets: Buckets,
    #[deku(ctx = "header, file_headers, file_names")]
    pub(crate) file_data: T,
}

impl<T: for<'a> DekuWriter<Ctx<'a>> + for<'a> DekuReader<'a, Ctx<'a>>> BND4<T> {
    fn read_file_names<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        unicode: bool,
        file_count: i32,
    ) -> Result<Vec<String>, DekuError> {
        let mut file_names = Vec::new();
        if unicode {
            for _ in 0..file_count as usize {
                let mut buffer = Vec::new();
                let mut char = u16::from_reader_with_ctx(reader, ())?;
                while char != 0 {
                    buffer.push(char);
                    char = u16::from_reader_with_ctx(reader, ())?;
                }
                file_names.push(String::from_utf16_lossy(&buffer));
            }
        } else {
            let mut file_names = Vec::new();
            for _ in 0..file_count as usize {
                let mut buffer = Vec::new();
                let mut char = u8::from_reader_with_ctx(reader, ())?;
                while char != 0 {
                    buffer.push(char);
                    char = u8::from_reader_with_ctx(reader, ())?;
                }
                let (file_name, _, _) = SHIFT_JIS.decode(&buffer);
                file_names.push(file_name.to_string());
            }
        }
        Ok(file_names)
    }
    fn write_file_names<W: std::io::Write>(
        writer: &mut deku::writer::Writer<W>,
        unicode: bool,
        file_names: &Vec<String>,
    ) -> Result<(), DekuError> {
        if unicode {
            for name in file_names {
                let mut str_utf16 = name.encode_utf16();
                let mut bytes = Vec::new();
                while let Some(wchar) = str_utf16.next() {
                    bytes.extend(wchar.to_le_bytes());
                }
                bytes.to_writer(writer, ())?;
            }
        } else {
            for name in file_names {
                let (bytes, _, _) = SHIFT_JIS.encode(&name);
                bytes.to_writer(writer, ())?;
            }
        }
        Ok(())
    }
}
