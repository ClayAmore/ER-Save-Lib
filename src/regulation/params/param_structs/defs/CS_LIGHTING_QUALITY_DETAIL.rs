use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_LIGHTING_QUALITY_DETAIL {
	pub localLightDistFactor: f32,
	pub localLightShadowEnabled: u8,
	pub forwardPassLightingEnabled: u8,
	pub localLightShadowSpecLevelMax: u8,
	pub dmy: [u8;1],
}
