use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WORLD_MAP_PLACE_NAME_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub worldMapPieceId: i32,
	pub textId: i32,
	pub pad1: [u8;4],
	pub areaNo: u8,
	pub gridXNo: u8,
	pub gridZNo: u8,
	pub pad2: [u8;1],
	pub posX: f32,
	pub posY: f32,
	pub posZ: f32,
}
