use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct KEY_ASSIGN_PARAM_ST {
	pub padKeyId: i32,
	pub keyboardModifyKey: i32,
	pub keyboardKeyId: i32,
	pub mouseModifyKey: i32,
	pub mouseKeyId: i32,
	#[deku(count = "12")]
	pub reserved: Vec<u8>,
}
