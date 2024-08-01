use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WAYPOINT_PARAM_ST {
	pub attribute1: i16,
	pub attribute2: i16,
	pub attribute3: i16,
	pub attribute4: i16,
	pub padding4: [u8;8],
}
