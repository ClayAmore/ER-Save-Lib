use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CAMERA_FADE_PARAM_ST {
	pub NearMinDist: f32,
	pub NearMaxDist: f32,
	pub FarMinDist: f32,
	pub FarMaxDist: f32,
	pub MiddleAlpha: f32,
	#[deku(count = "12")]
	pub dummy: Vec<u8>,
}
