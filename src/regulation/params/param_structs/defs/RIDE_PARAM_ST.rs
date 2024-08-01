use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct RIDE_PARAM_ST {
	pub atkChrId: i32,
	pub defChrId: i32,
	pub rideCamParamId: i32,
	pub atkChrAnimId: i32,
	pub defChrAnimId: i32,
	pub defAdjustDmyId: i32,
	pub defCheckDmyId: i32,
	pub diffAngMyToDef: f32,
	pub dist: f32,
	pub upperYRange: f32,
	pub lowerYRange: f32,
	pub diffAngMin: f32,
	pub diffAngMax: f32,
	#[deku(count = "12")]
	pub pad: Vec<u8>,
}
