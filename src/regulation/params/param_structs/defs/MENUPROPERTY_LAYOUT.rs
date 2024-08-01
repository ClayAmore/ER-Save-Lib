use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MENUPROPERTY_LAYOUT {
	#[deku(count = "16")]
	pub LayoutPath: Vec<u8>,
	pub PropertyID: i32,
	pub CaptionTextID: i32,
	pub HelpTextID: i32,
	pub reserved: [u8;4],
}
