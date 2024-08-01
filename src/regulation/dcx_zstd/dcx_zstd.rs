use std::borrow::Cow;
use std::io::Cursor;

use deku::ctx::Endian;
use deku::prelude::*;
use deku::{DekuRead, DekuWrite};

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(magic = b"DCX\0", endian = "Endian::Big")]
pub(crate) struct DCXZSTD<T: DekuWriter + for<'a> DekuReader<'a>> {
    #[deku(assert_eq = "0x11000")]
    unk0x4: i32,
    #[deku(assert_eq = "0x18")]
    unk0x8: i32,
    #[deku(assert_eq = "0x24")]
    unk0xc: i32,
    #[deku(assert_eq = "0x44")]
    unk0x10: i32,
    #[deku(assert_eq = "0x4c")]
    unk0x14: i32,
    #[deku(assert = "unk0x18 == b\"DCS\\0\"")]
    unk0x18: [u8; 4],
    decompressed_size: i32,
    compressed_size: i32,
    #[deku(assert = "unk0x24 == b\"DCP\\0\"")]
    unk0x24: [u8; 4],
    #[deku(assert = "unk0x28 == b\"ZSTD\"")]
    unk0x28: [u8; 4],
    #[deku(assert_eq = "0x20")]
    unk0x2c: i32,
    unk0x30: u8,
    #[deku(assert_eq = "0")]
    unk0x31: u8,
    #[deku(assert_eq = "0")]
    unk0x32: u8,
    #[deku(assert_eq = "0")]
    unk0x33: u8,
    #[deku(assert_eq = "0")]
    unk0x34: i32,
    #[deku(assert_eq = "0")]
    unk0x38: u8,
    #[deku(assert_eq = "0")]
    unk0x39: u8,
    #[deku(assert_eq = "0")]
    unk0x3a: u8,
    #[deku(assert_eq = "0")]
    unk0x3b: u8,
    #[deku(assert_eq = "0")]
    unk0x3c: i32,
    #[deku(assert_eq = "0x00010100")]
    unk0x40: i32,
    #[deku(assert = "unk0x44 == b\"DCA\\0\"")]
    unk0x44: [u8; 4],
    #[deku(assert_eq = "8")]
    unk0x48: i32,
    #[deku(
        reader = "DCXZSTD::decompress(deku::reader, *compressed_size)",
        writer = "DCXZSTD::compress(data, *compressed_size)"
    )]
    pub(crate) data: T,
}

impl<T: DekuWriter + for<'a> DekuReader<'a>> DCXZSTD<T> {
    pub(crate) fn decompress<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        size: i32,
    ) -> Result<T, DekuError> {
        let size = size as usize;
        let mut bytes = vec![0; size];
        let _ = reader.read_bytes(size, &mut bytes);

        // Try to decompress DCX ZSTD
        let result = zstd::decode_all(bytes.as_slice());

        // Throw error if decompression failed
        if let Err(err) = result {
            return Err(DekuError::Parse(Cow::from(format!(
                "Failed to decompress zstd! {err})"
            ))));
        };

        let bytes = result.unwrap();

        let mut cursor = Cursor::new(&bytes);
        let mut reader = Reader::new(&mut cursor);
        Ok(T::from_reader_with_ctx(&mut reader, ())?)
    }

    // Due to the zstd compression configuration not being known, it's not
    // possible to compress the regulation back to how it originally was in the save
    // file.
    pub(crate) fn compress(_: &T, _: i32) -> Result<Vec<u8>, DekuError> {
        todo!()
    }
}
