use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SP_EFFECT_SET_PARAM_ST {
	pub spEffectId1: i32,
	pub spEffectId2: i32,
	pub spEffectId3: i32,
	pub spEffectId4: i32,
}
