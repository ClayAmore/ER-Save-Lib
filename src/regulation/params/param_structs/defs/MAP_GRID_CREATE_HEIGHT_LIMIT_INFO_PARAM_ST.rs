use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST {
	pub GridEnableCreateHeightMin: f32,
	pub GridEnableCreateHeightMax: f32,
	#[deku(count = "24")]
	pub Reserve: Vec<u8>,
}
