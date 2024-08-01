use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_SHADOW_QUALITY_DETAIL {
	pub enabled: u8,
	pub maxFilterLevel: u8,
	pub dmy: [u8;2],
	pub textureSizeScaler: i32,
	pub textureSizeDivider: i32,
	pub textureMinSize: i32,
	pub textureMaxSize: i32,
	pub blurCountBias: i32,
}
