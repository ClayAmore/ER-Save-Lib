use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct RUNTIME_BONE_CONTROL_PARAM_ST {
	pub chrId: i32,
	pub ctrlType: u8,
	#[deku(count = "11")]
	pub pad: Vec<u8>,
	#[deku(count = "32")]
	pub applyBone: Vec<u8>,
	#[deku(count = "32")]
	pub targetBone1: Vec<u8>,
	#[deku(count = "32")]
	pub targetBone2: Vec<u8>,
}
