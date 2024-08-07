use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CUTSCENE_TEXTURE_LOAD_PARAM_ST {
	#[deku(bits = 6)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_Debug: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	#[deku(count = "16")]
	pub texName_00: Vec<u8>,
	#[deku(count = "16")]
	pub texName_01: Vec<u8>,
	#[deku(count = "16")]
	pub texName_02: Vec<u8>,
	#[deku(count = "16")]
	pub texName_03: Vec<u8>,
	#[deku(count = "16")]
	pub texName_04: Vec<u8>,
	#[deku(count = "16")]
	pub texName_05: Vec<u8>,
	#[deku(count = "16")]
	pub texName_06: Vec<u8>,
	#[deku(count = "16")]
	pub texName_07: Vec<u8>,
	#[deku(count = "16")]
	pub texName_08: Vec<u8>,
	#[deku(count = "16")]
	pub texName_09: Vec<u8>,
	#[deku(count = "16")]
	pub texName_10: Vec<u8>,
	#[deku(count = "16")]
	pub texName_11: Vec<u8>,
	#[deku(count = "16")]
	pub texName_12: Vec<u8>,
	#[deku(count = "16")]
	pub texName_13: Vec<u8>,
	#[deku(count = "16")]
	pub texName_14: Vec<u8>,
	#[deku(count = "16")]
	pub texName_15: Vec<u8>,
}
