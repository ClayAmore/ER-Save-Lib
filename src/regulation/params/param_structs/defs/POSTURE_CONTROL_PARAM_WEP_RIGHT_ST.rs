use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct POSTURE_CONTROL_PARAM_WEP_RIGHT_ST {
	pub a000_rightArmFB: i16,
	pub a000_rightWristFB: i16,
	pub a000_rightWristIO: i16,
	pub a000_leftArmFB: i16,
	pub a000_leftWristFB: i16,
	pub a000_leftWristIO: i16,
	pub a002_rightArmFB: i16,
	pub a002_rightWristFB: i16,
	pub a002_rightWristIO: i16,
	pub a002_leftArmFB: i16,
	pub a002_leftWristFB: i16,
	pub a002_leftWristIO: i16,
	pub a003_rightArmFB: i16,
	pub a003_rightWristFB: i16,
	pub a003_rightWristIO: i16,
	pub a003_leftArmFB: i16,
	pub a003_leftWristFB: i16,
	pub a003_leftWristIO: i16,
	pub a010_rightArmFB: i16,
	pub a010_rightWristFB: i16,
	pub a010_rightWristIO: i16,
	pub a010_leftArmFB: i16,
	pub a010_leftWristFB: i16,
	pub a010_leftWristIO: i16,
	pub a012_rightArmFB: i16,
	pub a012_rightWristFB: i16,
	pub a012_rightWristIO: i16,
	pub a012_leftArmFB: i16,
	pub a012_leftWristFB: i16,
	pub a012_leftWristIO: i16,
	pub a013_rightArmFB: i16,
	pub a013_rightWristFB: i16,
	pub a013_rightWristIO: i16,
	pub a013_leftArmFB: i16,
	pub a013_leftWristFB: i16,
	pub a013_leftWristIO: i16,
	pub a014_rightArmFB: i16,
	pub a014_rightWristFB: i16,
	pub a014_rightWristIO: i16,
	pub a014_leftArmFB: i16,
	pub a014_leftWristFB: i16,
	pub a014_leftWristIO: i16,
	pub a015_rightArmFB: i16,
	pub a015_rightWristFB: i16,
	pub a015_rightWristIO: i16,
	pub a015_leftArmFB: i16,
	pub a015_leftWristFB: i16,
	pub a015_leftWristIO: i16,
	pub a016_rightArmFB: i16,
	pub a016_rightWristFB: i16,
	pub a016_rightWristIO: i16,
	pub a016_leftArmFB: i16,
	pub a016_leftWristFB: i16,
	pub a016_leftWristIO: i16,
	#[deku(skip, cond = "version >= 11210015")]
	pub pad: [u8;4],
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x6c: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x70: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x74: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x78: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x7c: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x80: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x84: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x88: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub endPad: [u8;4],
}
