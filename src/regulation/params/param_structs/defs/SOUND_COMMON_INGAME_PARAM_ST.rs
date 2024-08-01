use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SOUND_COMMON_INGAME_PARAM_ST {
	#[deku(count = "32")]
	pub ParamKeyStr: Vec<u8>,
	#[deku(count = "32")]
	pub ParamValueStr: Vec<u8>,
}
