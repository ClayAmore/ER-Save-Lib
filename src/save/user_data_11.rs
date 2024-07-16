use std::io::Cursor;

use deku::prelude::*;
use deku::ctx::Endian;
use deku::{DekuRead, DekuWrite};

use super::util::Util;


#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, start: usize, size: usize, is_ps: bool")]
pub(crate) struct UserData11 {
    // Checksum (PC only)
    #[deku(skip, cond = "is_ps", count = "0x10")]
    checksum: Vec<u8>,

    unk0x10: [u8; 0x10],

    #[deku(count = "0x1e9fb0")]
    regulation: Vec<u8>,

    #[deku( count = "size - (deku::byte_offset - start)" )]
    pub(crate) rest: Vec<u8>
}

impl UserData11 {
    pub(crate) fn read<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        endian: Endian, 
        start: usize, 
        size: usize, 
        is_ps: bool
    ) -> Result<Self, DekuError> {
        let user_data_11 = Self::from_reader_with_ctx(reader, (endian, start, size, is_ps))?;
        Ok(user_data_11)
    }

    pub(crate) fn write<W: std::io::Write>(
        writer: &mut deku::writer::Writer<W>,
        endian: Endian, 
        start: usize,
        size: usize, 
        is_ps: bool,
        user_data_11: &Self
    ) -> Result<(), DekuError> {
        
        if is_ps {
            user_data_11.to_writer(writer, (endian, start, size, is_ps))?;
            return Ok(());
        }

        let mut buffer = Vec::new();
        {
            let mut temp_writer = Writer::new(Cursor::new(&mut buffer));
            user_data_11.to_writer(&mut temp_writer, (endian, start, size, is_ps))?;
        }
        
        Util::update_checksum(&mut buffer);

        writer.write_bytes(&buffer)?;
        Ok(())
    }
}