use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct PERFORMANCE_CHECK_PARAM {
	pub workTag: u8,
	pub categoryTag: u8,
	pub compareType: u8,
	pub dummy1: [u8;1],
	pub compareValue: f32,
	pub dummy2: [u8;8],
	#[deku(count = "16")]
	pub userTag: Vec<u8>,
}
