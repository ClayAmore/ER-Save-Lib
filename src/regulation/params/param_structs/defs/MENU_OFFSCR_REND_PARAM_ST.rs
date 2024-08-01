use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MENU_OFFSCR_REND_PARAM_ST {
	pub camAtPosX: f32,
	pub camAtPosY: f32,
	pub camAtPosZ: f32,
	pub camDist: f32,
	pub camRotX: f32,
	pub camRotY: f32,
	pub camFov: f32,
	pub camDistMin: f32,
	pub camDistMax: f32,
	pub camRotXMin: f32,
	pub camRotXMax: f32,
	pub GparamID: i32,
	pub envTexId: i32,
	pub Grapm_ID_forPS4: i32,
	pub Grapm_ID_forXB1: i32,
	pub pad: [u8;4],
}
