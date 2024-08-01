use super::flags::Format;
use deku::prelude::*;
use deku::DekuError;

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(ctx = "format: u8, bit_big_endian: bool")]
pub(crate) struct BND4FileHeader {
    #[deku(
        map = "|b: u8| -> Result<_, DekuError> {if bit_big_endian {Ok(b)} else {Ok(b.reverse_bits())}}"
    )]
    pub(crate) file_flags: u8,
    #[deku(assert_eq = "0")]
    unk0x1: u8,
    #[deku(assert_eq = "0")]
    unk0x2: u8,
    #[deku(assert_eq = "0")]
    unk0x3: u8,
    #[deku(assert_eq = "-1")]
    unk0x4: i32,
    pub(crate) compressed_size: i64,
    #[deku(skip, cond = "format & Format::Compression as u8 == 0")]
    pub(crate) uncompressed_size: i64,
    #[deku(
        reader = "BND4FileHeader::read_data_offset(deku::reader, format)",
        writer = "BND4FileHeader::write_data_offset(deku::writer, format, *data_offset)"
    )]
    pub(crate) data_offset: i64,
    #[deku(skip, cond = "format & Format::IDs as u8 == 0")]
    pub(crate) id: i32,
    #[deku(
        skip,
        cond = "format & (Format::Names1 as u8 | Format::Names2 as u8) == 0"
    )]
    pub(crate) name_offset: i32,
    #[deku(skip, cond = "format != Format::Names1 as u8", assert_eq = "0")]
    unk0x28: i32,
}

impl BND4FileHeader {
    fn read_data_offset<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        format: u8,
    ) -> Result<i64, DekuError> {
        let has_long_offset = format & Format::LongOffsets as u8 != 0;

        if has_long_offset {
            i64::from_reader_with_ctx(reader, ())
        } else {
            let res = i32::from_reader_with_ctx(reader, ())?;
            Ok(res as i64)
        }
    }
    fn write_data_offset<W: std::io::Write>(
        writer: &mut deku::writer::Writer<W>,
        format: u8,
        data_offset: i64,
    ) -> Result<(), DekuError> {
        let has_long_offset = format & Format::LongOffsets as u8 != 0;
        if has_long_offset {
            data_offset.to_writer(writer, ())
        } else {
            (data_offset as i32).to_writer(writer, ())
        }
    }
}
