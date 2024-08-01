use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SIGN_PUDDLE_TAB_PARAM_ST {
	pub isDlcTab: i32,
	pub tabTextId: i32,
	pub unknown_0x8: i32,
	pub unknown_0xc: i32,
}
