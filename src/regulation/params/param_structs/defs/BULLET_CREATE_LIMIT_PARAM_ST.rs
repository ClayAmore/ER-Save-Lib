use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct BULLET_CREATE_LIMIT_PARAM_ST {
	pub limitNum_byGroup: u8,
	#[deku(bits = 7)]
	pub pad2: u8,
	#[deku(bits = 1)]
	pub isLimitEachOwner: u8,
	#[deku(count = "30")]
	pub pad: Vec<u8>,
}
