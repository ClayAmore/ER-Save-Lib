use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct COMMON_SYSTEM_PARAM_ST {
	pub mapSaveMapNameIdOnGameStart: i32,
	#[deku(count = "60")]
	pub reserve0: Vec<u8>,
}
