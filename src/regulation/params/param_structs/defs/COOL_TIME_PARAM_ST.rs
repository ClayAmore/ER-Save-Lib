use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct COOL_TIME_PARAM_ST {
	pub limitationTime_0: f32,
	pub observeTime_0: f32,
	pub limitationTime_1: f32,
	pub observeTime_1: f32,
	pub limitationTime_2: f32,
	pub observeTime_2: f32,
	pub limitationTime_3: f32,
	pub observeTime_3: f32,
}
