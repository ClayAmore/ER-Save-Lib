use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST {
	pub DrawDist_LvBegin: u8,
	pub DrawDist_LvEnd: u8,
	pub reserve0: [u8;2],
	pub DrawDist_ScaleBegin: f32,
	pub DrawDist_ScaleEnd: f32,
	pub ShadwDrawDist_LvBegin: u8,
	pub ShadwDrawDist_LvEnd: u8,
	pub reserve1: [u8;2],
	pub ShadwDrawDist_ScaleBegin: f32,
	pub ShadwDrawDist_ScaleEnd: f32,
	#[deku(count = "24")]
	pub reserve2: Vec<u8>,
}
