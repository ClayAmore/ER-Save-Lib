use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct GPARAM_GRID_REGION_INFO_PARAM_ST {
	pub GparamGridRegionId: i32,
	#[deku(count = "28")]
	pub Reserve: Vec<u8>,
}
