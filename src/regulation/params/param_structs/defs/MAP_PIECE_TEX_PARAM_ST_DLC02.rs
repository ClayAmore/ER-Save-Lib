use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MAP_PIECE_TEX_PARAM_ST_DLC02 {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub srcR: u8,
	pub srcG: u8,
	pub srcB: u8,
	pub pad1: [u8;1],
	pub saveMapNameId: i32,
	pub multiPlayAreaId: i32,
	pub unknown_0x10: i32,
	pub unknown_0x14: i32,
	pub unknown_0x18: i32,
	pub unknown_0x1c: i32,
	pub unknownPlayRegion_1: i32,
	pub unknownPlayRegion_2: i32,
}
