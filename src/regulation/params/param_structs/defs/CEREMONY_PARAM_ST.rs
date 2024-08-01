use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CEREMONY_PARAM_ST {
	pub eventLayerId: i32,
	pub mapStudioLayerId: i32,
	pub multiPlayAreaOffset: i32,
	pub overrideMapPlaceNameId: i32,
	pub overrideSaveMapNameId: i32,
	#[deku(count = "16")]
	pub pad2: Vec<u8>,
}
