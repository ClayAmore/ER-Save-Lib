use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MENU_PARAM_COLOR_TABLE_ST {
	pub lerpMode: u8,
	pub pad1: [u8;3],
	pub h: i16,
	pub pad2: [u8;2],
	pub s1: f32,
	pub v1: f32,
	pub s2: f32,
	pub v2: f32,
	pub s3: f32,
	pub v3: f32,
}
