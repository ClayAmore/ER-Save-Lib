use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct DIRECTION_CAMERA_PARAM_ST {
	#[deku(bits = 1)]
	pub isUseOption: u8,
	#[deku(bits = 3)]
	pub pad2: u8,
	#[deku(count = "15")]
	pub pad1: Vec<u8>,
}
