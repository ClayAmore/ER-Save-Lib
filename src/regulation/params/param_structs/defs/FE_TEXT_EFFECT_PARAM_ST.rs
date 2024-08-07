use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct FE_TEXT_EFFECT_PARAM_ST {
	pub resId: i16,
	pub pad1: [u8;2],
	pub textId: i32,
	pub seId: i32,
	#[deku(bits = 7)]
	pub pad3: u8,
	#[deku(bits = 1)]
	pub canMixMapName: u8,
	#[deku(count = "19")]
	pub pad2: Vec<u8>,
}
