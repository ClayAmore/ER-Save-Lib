use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct TUTORIAL_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub menuType: u8,
	pub triggerType: u8,
	pub repeatType: u8,
	pub pad1: [u8;1],
	pub imageId: i16,
	pub pad2: [u8;2],
	pub unlockEventFlagId: i32,
	pub textId: i32,
	pub displayMinTime: f32,
	pub displayTime: f32,
	pub pad3: [u8;4],
}
