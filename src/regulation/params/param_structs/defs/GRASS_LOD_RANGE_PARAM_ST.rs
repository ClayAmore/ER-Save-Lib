use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct GRASS_LOD_RANGE_PARAM_ST {
	pub LOD0_range: f32,
	pub LOD0_play: f32,
	pub LOD1_range: f32,
	pub LOD1_play: f32,
	pub LOD2_range: f32,
	pub LOD2_play: f32,
}
