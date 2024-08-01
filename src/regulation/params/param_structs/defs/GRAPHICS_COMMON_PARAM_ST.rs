use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct GRAPHICS_COMMON_PARAM_ST {
	pub hitBulletDecalOffset_HitIns: f32,
	pub reserved02: [u8;8],
	pub charaWetDecalFadeRange: f32,
	#[deku(count = "240")]
	pub reserved04: Vec<u8>,
}
