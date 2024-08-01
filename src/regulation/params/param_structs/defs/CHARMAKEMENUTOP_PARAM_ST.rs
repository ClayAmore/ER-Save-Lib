use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CHARMAKEMENUTOP_PARAM_ST {
	pub commandType: i32,
	pub captionId: i32,
	pub faceParamId: i32,
	pub tableId: i32,
	pub viewCondition: i32,
	pub previewMode: i8,
	pub reserved2: [u8;3],
	pub tableId2: i32,
	pub refFaceParamId: i32,
	pub refTextId: i32,
	pub helpTextId: i32,
	pub unlockEventFlagId: i32,
	pub reserved: [u8;4],
}
