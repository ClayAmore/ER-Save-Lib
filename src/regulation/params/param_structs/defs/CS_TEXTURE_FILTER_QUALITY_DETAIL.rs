use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_TEXTURE_FILTER_QUALITY_DETAIL {
	pub filter: u8,
	pub dmy: [u8;3],
	pub maxAnisoLevel: i32,
}
