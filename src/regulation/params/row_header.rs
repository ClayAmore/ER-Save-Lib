use deku::{ctx::Endian, reader::Reader, writer::Writer, DekuError, DekuReader, DekuWriter};

use super::{flags::FormatFlags, params::Offset};

// Context params for ParamRowHeader
type Ctx = (Endian, u8, u8);

// PARAM Row Header
#[derive(Debug, PartialEq)]
pub(crate) struct ParamRowHeader {
    pub(crate) id: i32,
    pub(crate) unk0x4: i32,
    pub(crate) data_offset: Offset,
    pub(crate) name_offset: Offset,
}
impl<'a> DekuReader<'a, Ctx> for ParamRowHeader {
    fn from_reader_with_ctx<R: std::io::Read>(
        reader: &mut Reader<R>,
        ctx: Ctx,
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        // Context
        let (endian, format0x2d, _) = ctx;

        // ID | 0x0
        let id = i32::from_reader_with_ctx(reader, endian)?;
        let (unk0x4, data_offset, name_offset) =
            if format0x2d & FormatFlags::LongDataOffset as u8 != 0 {
                // Padding | 0x4
                let unk0x4 = i32::from_reader_with_ctx(reader, ())?;
                // Padding | 0x8
                let data_offset = Offset::Long(i64::from_reader_with_ctx(reader, ())?);
                // Padding | 0x10
                let name_offset = Offset::Long(i64::from_reader_with_ctx(reader, ())?);
                (unk0x4, data_offset, name_offset)
            } else {
                // Padding | 0x4
                let data_offset = Offset::Int(i32::from_reader_with_ctx(reader, ())?);
                // Padding | 0x8
                let name_offset = Offset::Int(i32::from_reader_with_ctx(reader, ())?);
                (0, data_offset, name_offset)
            };

        Ok(ParamRowHeader {
            id,
            unk0x4,
            data_offset,
            name_offset,
        })
    }
}
impl DekuWriter<Ctx> for ParamRowHeader {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut Writer<W>,
        ctx: Ctx,
    ) -> Result<(), DekuError> {
        let (endian, _, _) = ctx;

        // ID | 0x0
        self.id.to_writer(writer, endian)?;

        match self.data_offset {
            Offset::None => {}
            Offset::Long(data_offset) => {
                // padding | 0x4
                self.unk0x4.to_writer(writer, endian)?;
                // Data Offset | 0x8
                data_offset.to_writer(writer, endian)?;
            }
            Offset::Int(data_offset) => {
                // Data Offset | 0x4
                data_offset.to_writer(writer, endian)?;
            }
        }
        match self.name_offset {
            Offset::None => {}
            Offset::Long(name_offset) => {
                // Name Offset | 0x10
                name_offset.to_writer(writer, endian)?;
            }
            Offset::Int(name_offset) => {
                // Name Offset | 0x8
                name_offset.to_writer(writer, endian)?;
            }
        }

        Ok(())
    }
}
