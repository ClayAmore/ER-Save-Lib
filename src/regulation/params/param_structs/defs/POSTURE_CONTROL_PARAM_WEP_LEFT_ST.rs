use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct POSTURE_CONTROL_PARAM_WEP_LEFT_ST {
	pub a000_leftArmFB: i16,
	pub a000_leftWristFB: i16,
	pub a000_leftWristIO: i16,
	pub a002_leftArmFB: i16,
	pub a002_leftWristFB: i16,
	pub a002_leftWristIO: i16,
	pub a003_leftArmFB: i16,
	pub a003_leftWristFB: i16,
	pub a003_leftWristIO: i16,
	#[deku(skip, cond = "version >= 11210015", count = "14")]
	pub pad_old: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x12: i16,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x14: i16,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x16: i16,
	#[deku(skip, cond = "version < 11210015")]
	pub pad: [u8;8],
}
