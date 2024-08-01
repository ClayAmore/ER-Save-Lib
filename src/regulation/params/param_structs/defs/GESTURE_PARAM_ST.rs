use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct GESTURE_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub itemId: i32,
	pub msgAnimId: i32,
	#[deku(bits = 1)]
	pub cannotUseRiding: u8,
	#[deku(bits = 7)]
	pub pad2: u8,
	pub pad1: [u8;3],
}
