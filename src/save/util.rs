use std::borrow::Cow;

use deku::{DekuError, DekuWriter};

// Custom types
// =================================
pub(crate) type FloatVector3 = (f32, f32, f32);
pub(crate) type FloatVector4 = (f32, f32, f32, f32);
pub(crate) type MapId = [u8; 4];
// =================================

pub(crate) struct Util;

impl Util {
    pub(crate) fn read_wstring<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        size: usize,
    ) -> Result<String, DekuError> {
        if size % 2 != 0 {
            return Err(DekuError::Parse(Cow::from(
                "Size is not even. Cannot parse bytes into u16 string",
            )));
        }

        let mut buffer = vec![0; size];
        let _ = reader.read_bytes(size, &mut buffer)?;

        if buffer.as_ptr() as usize % std::mem::align_of::<u16>() != 0 {
            return Err(DekuError::Parse(Cow::from(
                "Unaligned. Cannot parse bytes into u16 string",
            )));
        }

        let mut u16_vec: Vec<u16> = Vec::new();
        let mut iter = buffer.into_iter();
        while let (Some(b1), Some(b2)) = (iter.next(), iter.next()) {
            if b1 == 0 && b2 == 0 {
                break;
            }
            u16_vec.push(u16::from_le_bytes([b1, b2]));
        }

        let str = String::from_utf16(&u16_vec);

        match str {
            Ok(str) => Ok(str.trim_end().to_string()),
            Err(err) => Err(DekuError::Parse(Cow::from(format!("{err}")))),
        }
    }

    pub(crate) fn write_wstring<R: std::io::Write>(
        writer: &mut deku::writer::Writer<R>,
        str: &String,
        size: isize,
    ) -> Result<(), DekuError> {
        let mut str_utf16 = str.encode_utf16();
        let mut bytes = Vec::new();

        let mut count = 0;
        while let Some(wchar) = str_utf16.next() {
            bytes.extend(wchar.to_le_bytes());
            count += 2;
            if count == size {
                break;
            }
        }

        let remaining = size - count;
        for _ in 0..remaining {
            bytes.push(0);
        }

        bytes.to_writer(writer, ())?;

        return Ok(());
    }

    pub(crate) fn update_checksum(bytes: &mut Vec<u8>) {
        let digest = md5::compute(&bytes[0x10..]);
        for (i, byte) in digest.0.iter().enumerate() {
            bytes[i] = *byte;
        }
    }
}
