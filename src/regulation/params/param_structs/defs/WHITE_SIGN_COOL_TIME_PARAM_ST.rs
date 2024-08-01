use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WHITE_SIGN_COOL_TIME_PARAM_ST {
	pub limitationTime_Normal: f32,
	pub limitationTime_NormalDriedFinger: f32,
	pub limitationTime_Guardian: f32,
	pub limitationTime_GuardianDriedFinger: f32,
}
