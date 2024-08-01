use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct RESIST_CORRECT_PARAM_ST {
	pub addPoint1: f32,
	pub addPoint2: f32,
	pub addPoint3: f32,
	pub addPoint4: f32,
	pub addPoint5: f32,
	pub addRate1: f32,
	pub addRate2: f32,
	pub addRate3: f32,
	pub addRate4: f32,
	pub addRate5: f32,
}
