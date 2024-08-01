use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CHARMAKEMENU_LISTITEM_PARAM_ST {
	pub value: i32,
	pub captionId: i32,
	pub iconId: u8,
	pub reserved: [u8;7],
}
