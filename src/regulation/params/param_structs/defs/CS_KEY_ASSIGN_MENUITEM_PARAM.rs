use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_KEY_ASSIGN_MENUITEM_PARAM {
	pub textID: i32,
	pub key: i32,
	pub enableUnassign: u8,
	pub enablePadConfig: u8,
	pub enableMouseConfig: u8,
	pub group: u8,
	pub mappingTextID: i32,
	pub viewPad: u8,
	pub viewKeyboardMouse: u8,
	pub padding: [u8;6],
}
