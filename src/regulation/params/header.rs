use std::io::Cursor;

use deku::{ctx::Endian, reader::Reader, writer::Writer, DekuError, DekuReader, DekuWriter};
use encoding_rs::SHIFT_JIS;

use crate::regulation::params::flags::FormatFlags;

use super::params::{Offset, ParamType};

// PARAM Header
#[derive(PartialEq, Debug)]
pub(crate) struct PARAMHeader {
    pub(crate) string_offset: u32,
    pub(crate) unk0x4: u16,
    pub(crate) unk0x6: i16,
    pub(crate) param_def_format_version: u8,
    pub(crate) row_count: u16,
    pub(crate) param_type: ParamType,
    pub(crate) endian: Endian,
    pub(crate) format0x2d: u8,
    pub(crate) format0x2e: u8,
    pub(crate) param_def_data_version: i16,
    pub(crate) data_offset: Offset,
}
impl<'a> DekuReader<'a> for PARAMHeader {
    fn from_reader_with_ctx<R: std::io::Read>(
        reader: &mut Reader<R>,
        _: (),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        // Need to do this to detemrine endianess and get flags at 0x2d and 0x2e
        let mut header = [0; 0x40];
        reader.read_bytes(0x40, &mut header)?;

        // Endianess
        assert!(header[0x2c] == 0 || header[0x2c] == 0xff);

        // Endian | 0x2c
        let endian = if header[0x2c] == 0xff {
            Endian::Big
        } else {
            Endian::Little
        };

        // Format Flags | 0x2D
        let format0x2d = header[0x2d];

        // Format Flags | 0x2E
        let format0x2e = header[0x2e];

        // Param Def Format Version | 0x2F
        let param_def_format_version = header[0x2f];

        // Make header into reader again
        let mut cursor = Cursor::new(header);
        let mut header_reader = Reader::new(&mut cursor);

        // String Offset | 0x0
        let string_offset = u32::from_reader_with_ctx(&mut header_reader, endian)?;

        // ?? | 0x4
        let unk0x4 = if format0x2d & FormatFlags::Flag01 as u8 != 0
            && format0x2d & FormatFlags::IntDataOffset as u8 != 0
            || format0x2d & FormatFlags::LongDataOffset as u8 != 0
        {
            assert_eq!(i16::from_reader_with_ctx(&mut header_reader, endian)?, 0);
            0
        } else {
            u16::from_reader_with_ctx(&mut header_reader, endian)?
        };

        // Unk0x6 | 0x6
        let unk0x6 = i16::from_reader_with_ctx(&mut header_reader, endian)?;

        // Param Data Version | 0x8
        let param_def_data_version = i16::from_reader_with_ctx(&mut header_reader, endian)?;

        // Row Count | 0xA
        let row_count = u16::from_reader_with_ctx(&mut header_reader, endian)?;

        // Param Type | 0xC
        // Paramtype is either a string or an offset depending on how long the paramtype is.
        // ParamType as Offset.
        let param_type = if format0x2d & FormatFlags::OffsetParamType as u8 != 0 {
            // ?? | 0xC
            assert_eq!(i32::from_reader_with_ctx(&mut header_reader, ())?, 0);

            // ?? | 0x10
            let param_type_offset = i64::from_reader_with_ctx(&mut header_reader, ())?;

            // ?? | 0x18
            let mut buffer = vec![0; 0x14];
            header_reader.read_bytes(0x14, &mut buffer)?;
            assert_eq!(buffer, vec![0; 0x14]);

            ParamType::Offset(param_type_offset)
        }
        // Paramtype as String
        else {
            // ?? | 0xC
            let mut bytes = vec![0; 0x20];
            header_reader.read_bytes(0x20, &mut bytes)?;
            let (str, _, _) = SHIFT_JIS.decode(&mut bytes);
            ParamType::String(str.to_string())
        };

        // skip 4 bytes | 0x30
        u32::from_reader_with_ctx(&mut header_reader, ())?;

        let data_offset = if format0x2d & FormatFlags::Flag01 as u8 != 0
            && format0x2d & FormatFlags::IntDataOffset as u8 != 0
        {
            // Data Offset | 0x30
            let data_offset = i32::from_reader_with_ctx(&mut header_reader, ())?;

            // ?? | 0x34
            assert_eq!(i32::from_reader_with_ctx(&mut header_reader, ())?, 0);

            // ?? | 0x38
            assert_eq!(i32::from_reader_with_ctx(&mut header_reader, ())?, 0);

            // ?? | 0x3c
            assert_eq!(i32::from_reader_with_ctx(&mut header_reader, ())?, 0);

            Offset::Int(data_offset)
        } else if format0x2d & FormatFlags::LongDataOffset as u8 != 0 {
            // ?? | 0x30
            let data_offset = i64::from_reader_with_ctx(&mut header_reader, ())?;

            // ?? | 0x38
            assert_eq!(i64::from_reader_with_ctx(&mut header_reader, ())?, 0);

            Offset::Long(data_offset)
        } else {
            Offset::None
        };

        Ok(Self {
            string_offset,
            unk0x4,
            unk0x6,
            param_def_data_version,
            row_count,
            param_type,
            endian,
            format0x2d,
            format0x2e,
            param_def_format_version,
            data_offset,
        })
    }
}

impl DekuWriter for PARAMHeader {
    fn to_writer<W: std::io::Write>(&self, writer: &mut Writer<W>, _: ()) -> Result<(), DekuError> {
        // String offset | 0x0
        self.string_offset.to_writer(writer, self.endian)?;

        // ?? | 0x4
        self.unk0x4.to_writer(writer, self.endian)?;

        // ?? | 0x6
        self.unk0x6.to_writer(writer, self.endian)?;

        // Param Data Version | 0x8
        self.param_def_data_version.to_writer(writer, self.endian)?;

        // Row Count | 0xA
        self.row_count.to_writer(writer, self.endian)?;

        // ParamType or Offset | 0xC
        match &self.param_type {
            ParamType::String(str) => {
                let (bytes, _, _) = SHIFT_JIS.encode(str);
                bytes.to_writer(writer, self.endian)?;
                if bytes.len() < 0x20 {
                    let remaining = 0x20 - bytes.len();
                    vec![0u8; remaining].to_writer(writer, self.endian)?;
                }
            }
            ParamType::Offset(offset) => {
                0.to_writer(writer, self.endian)?;
                offset.to_writer(writer, self.endian)?;
                [0u8; 0x14].to_writer(writer, self.endian)?;
            }
        }

        // Big Endian | 0x2C
        match self.endian {
            Endian::Little => {
                0u8.to_writer(writer, self.endian)?;
            }
            Endian::Big => {
                (0xff as u8).to_writer(writer, self.endian)?;
            }
        }

        // Format Flags | 0x2D
        self.format0x2d.to_writer(writer, self.endian)?;

        // Format Flags | 0x2E
        self.format0x2e.to_writer(writer, self.endian)?;

        // Param Def Format Version | 0x2F
        self.param_def_format_version
            .to_writer(writer, self.endian)?;

        // Data Offset | 0x30
        match self.data_offset {
            Offset::None => {}
            Offset::Long(data_offset) => {
                // Param Def Format Version | 0x30
                data_offset.to_writer(writer, self.endian)?;
                // ?? | 0x34
                0.to_writer(writer, self.endian)?;
                // ?? | 0x38
                0.to_writer(writer, self.endian)?;
                // ?? | 0x3C
                0.to_writer(writer, self.endian)?;
            }
            Offset::Int(data_offset) => {
                // ?? | 0x30
                data_offset.to_writer(writer, self.endian)?;
                // ?? | 0x38
                (0 as i64).to_writer(writer, self.endian)?;
            }
        }

        Ok(())
    }
}
