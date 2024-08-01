use std::borrow::Cow;

use deku::prelude::*;
use deku::{ctx::Endian, DekuError};

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(magic = b"BND4")]
pub(crate) struct BND4Header {
    #[deku(assert_eq = "0")]
    unk0x4: u8,
    #[deku(assert_eq = "0")]
    unk0x5: u8,
    #[deku(assert_eq = "0")]
    unk0x6: u8,
    #[deku(assert_eq = "0")]
    unk0x7: u8,
    #[deku(assert_eq = "0")]
    unk0x8: u8,
    #[deku(
        reader = "BND4Header::read_endian(deku::reader)",
        writer = "BND4Header::write_endian(deku::writer, endian)"
    )]
    pub(crate) endian: Endian,
    #[deku(map = "|b: bool| -> Result<_, DekuError> {Ok(!b)}")]
    pub(crate) bit_big_endian: bool,
    #[deku(assert_eq = "0")]
    unk0x9: u8,
    pub(crate) file_count: i32,
    #[deku(assert_eq = "0x40")]
    pub(crate) header_size: i64,
    #[deku(
        reader = "BND4Header::read_version(deku::reader)",
        writer = "BND4Header::write_version(deku::writer, version)"
    )]
    pub(crate) version: u32,
    pub(crate) file_header_size: i64,
    pub(crate) headers_end: u64,
    pub(crate) unicode: bool,
    #[deku(reader = "BND4Header::read_format(deku::reader, bit_big_endian)")]
    pub(crate) format: u8,
    #[deku(assert = "*extended == 0 || *extended == 1 || *extended == 4 || *extended == 0x80")]
    pub(crate) extended: u8,
    #[deku(assert_eq = "0")]
    unk0x33: u8,
    #[deku(assert_eq = "0")]
    unk0x34: u32,
    #[deku(assert = "(*extended != 4 && *hashtable_offset == 0) || true")]
    pub(crate) hashtable_offset: u64,
}

impl BND4Header {
    fn read_format<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        bit_big_endian: &bool,
    ) -> Result<u8, DekuError> {
        let raw_format = u8::from_reader_with_ctx(reader, ())?;
        let reverse = *bit_big_endian || (raw_format & 1) != 0 && (raw_format & 0b1000_0000) == 0;
        if reverse {
            Ok(raw_format)
        } else {
            Ok(raw_format.reverse_bits())
        }
    }
    fn read_endian<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<Endian, DekuError> {
        let big_endian = bool::from_reader_with_ctx(reader, ())?;
        if big_endian {
            Ok(Endian::Big)
        } else {
            Ok(Endian::Little)
        }
    }

    fn write_endian<W: std::io::Write>(
        writer: &mut Writer<W>,
        endian: &Endian,
    ) -> Result<(), DekuError> {
        match endian {
            Endian::Little => 0.to_writer(writer, ()),
            Endian::Big => 1.to_writer(writer, ()),
        }
    }

    fn read_version<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
    ) -> Result<u32, DekuError> {
        // Prepare verison bytes buffer
        let mut bytes = vec![0u8; 8];

        // Read bytes into buffer
        reader.read_bytes(8, &mut bytes)?;

        // Try to convert bytes into string
        let version = String::from_utf8(bytes);
        if let Err(err) = version {
            return Err(DekuError::Parse(Cow::from(format!(
                "Failed to parse regulation version: {err})"
            ))));
        }

        // Try to parse string into u32
        let version = version.unwrap().parse::<u32>();
        if let Err(err) = version {
            return Err(DekuError::Parse(Cow::from(format!(
                "Failed to parse version as u32: {err})"
            ))));
        }

        Ok(version.unwrap())
    }

    fn write_version<W: std::io::Write>(
        writer: &mut Writer<W>,
        version: &u32,
    ) -> Result<(), DekuError> {
        // Convert u32 into string bytes
        let bytes = version.to_string().into_bytes();

        // Write string bytes
        bytes.to_writer(writer, ())?;

        Ok(())
    }
}
