use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MENU_VALUE_TABLE_SPEC {
	pub value: i32,
	pub textId: i32,
	pub compareType: i8,
	pub padding: [u8;3],
}
