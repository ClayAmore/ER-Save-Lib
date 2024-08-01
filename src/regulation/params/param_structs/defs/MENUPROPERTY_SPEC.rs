use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MENUPROPERTY_SPEC {
	pub CaptionTextID: i32,
	pub IconID: i32,
	pub RequiredPropertyID: i32,
	pub CompareType: i8,
	pub pad2: [u8;1],
	pub FormatType: i16,
	#[deku(count = "16")]
	pub pad: Vec<u8>,
}
