use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MULTI_PLAY_CORRECTION_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub client1SpEffectId: i32,
	pub client2SpEffectId: i32,
	pub client3SpEffectId: i32,
	pub bOverrideSpEffect: u8,
	#[deku(count = "15")]
	pub pad3: Vec<u8>,
}
