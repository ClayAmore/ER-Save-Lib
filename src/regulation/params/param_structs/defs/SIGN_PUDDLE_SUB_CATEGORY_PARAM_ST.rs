use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SIGN_PUDDLE_SUB_CATEGORY_PARAM_ST {
	pub startPad: [u8;4],
	pub signPuddleCategoryText: i32,
	pub signPuddleTabId: i16,
	pub unknown_0xa: i16,
	pub endPad: [u8;4],
}
