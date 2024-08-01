use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct FINAL_DAMAGE_RATE_PARAM_ST {
	pub physRate: f32,
	pub magRate: f32,
	pub fireRate: f32,
	pub thunRate: f32,
	pub darkRate: f32,
	pub staminaRate: f32,
	pub saRate: f32,
}
