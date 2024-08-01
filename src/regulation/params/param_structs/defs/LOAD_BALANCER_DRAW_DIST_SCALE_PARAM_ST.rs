use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST {
	pub Lv00: f32,
	pub Lv01: f32,
	pub Lv02: f32,
	pub Lv03: f32,
	pub Lv04: f32,
	pub Lv05: f32,
	pub Lv06: f32,
	pub Lv07: f32,
	pub Lv08: f32,
	pub Lv09: f32,
	pub Lv10: f32,
	pub Lv11: f32,
	pub Lv12: f32,
	pub Lv13: f32,
	pub Lv14: f32,
	pub Lv15: f32,
	pub Lv16: f32,
	pub Lv17: f32,
	pub Lv18: f32,
	pub Lv19: f32,
	pub Lv20: f32,
	#[deku(count = "44")]
	pub reserve: Vec<u8>,
}
