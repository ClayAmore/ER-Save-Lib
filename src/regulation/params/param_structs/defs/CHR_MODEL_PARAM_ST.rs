use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CHR_MODEL_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub modelMemoryType: u8,
	pub texMemoryType: u8,
	pub cameraDitherFadeId: i16,
	pub reportAnimMemSizeMb: f32,
	#[deku(skip, cond = "version < 10601000")]
	pub unk: i32,
}
