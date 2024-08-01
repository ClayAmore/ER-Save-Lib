use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CHR_EQUIP_MODEL_PARAM_ST {
	pub unknown_0x0: i32,
	pub unknown_0x4: i32,
	pub unknown_0x8: i32,
}
