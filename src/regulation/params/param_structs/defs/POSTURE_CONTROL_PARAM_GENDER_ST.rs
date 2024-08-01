use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct POSTURE_CONTROL_PARAM_GENDER_ST {
	pub a000_rightElbowIO: i16,
	pub a000_leftElbowIO: i16,
	pub a000_bothLegsIO: i16,
	pub a002_rightElbowIO: i16,
	pub a002_leftElbowIO: i16,
	pub a002_bothLegsIO: i16,
	pub a003_rightElbowIO: i16,
	pub a003_leftElbowIO: i16,
	pub a003_bothLegsIO: i16,
	pub a010_rightElbowIO: i16,
	pub a010_leftElbowIO: i16,
	pub a010_bothLegsIO: i16,
	pub a012_rightElbowIO: i16,
	pub a012_leftElbowIO: i16,
	pub a012_bothLegsIO: i16,
	pub a013_rightElbowIO: i16,
	pub a013_leftElbowIO: i16,
	pub a013_bothLegsIO: i16,
	pub a014_rightElbowIO: i16,
	pub a014_leftElbowIO: i16,
	pub a014_bothLegsIO: i16,
	pub a015_rightElbowIO: i16,
	pub a015_leftElbowIO: i16,
	pub a015_bothLegsIO: i16,
	pub a016_rightElbowIO: i16,
	pub a016_leftElbowIO: i16,
	pub a016_bothLegsIO: i16,
	#[deku(count = "10")]
	pub pad: Vec<u8>,
}
