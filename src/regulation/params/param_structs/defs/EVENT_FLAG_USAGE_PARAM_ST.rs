use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct EVENT_FLAG_USAGE_PARAM_ST {
	pub usageType: u8,
	pub playlogCategory: u8,
	pub padding1: [u8;2],
	pub flagNum: i32,
	#[deku(count = "24")]
	pub padding2: Vec<u8>,
}
