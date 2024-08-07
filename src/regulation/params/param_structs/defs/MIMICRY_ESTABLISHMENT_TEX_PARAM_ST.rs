use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MIMICRY_ESTABLISHMENT_TEX_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub srcR: u8,
	pub srcG: u8,
	pub srcB: u8,
	pub pad1: [u8;1],
	pub mimicryEstablishmentParamId: i32,
	pub pad2: [u8;4],
}
