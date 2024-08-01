use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_DOF_QUALITY_DETAIL {
	pub enabled: u8,
	pub dmy: [u8;3],
	pub forceHiResoBlur: i32,
	pub maxBlurLevel: i32,
}
