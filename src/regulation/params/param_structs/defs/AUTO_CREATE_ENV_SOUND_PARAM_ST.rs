use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct AUTO_CREATE_ENV_SOUND_PARAM_ST {
	pub RangeMin: f32,
	pub RangeMax: f32,
	pub LifeTimeMin: f32,
	pub LifeTimeMax: f32,
	pub DeleteDist: f32,
	pub NearDist: f32,
	pub LimiteRotateMin: f32,
	pub LimiteRotateMax: f32,
}
