use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SIGN_PUDDLE_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub matchAreaId: i32,
	#[deku(skip, cond = "version >= 11210015", count = "24")]
	pub pad1: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x20: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x24: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub areaNo: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub gridXNo: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub gridZNo: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub pad3: [u8;1],
	#[deku(skip, cond = "version < 11210015")]
	pub posX: f32,
	#[deku(skip, cond = "version < 11210015")]
	pub posY: f32,
	#[deku(skip, cond = "version < 11210015")]
	pub posZ: f32,
	#[deku(skip, cond = "version < 11210015")]
	pub signSubCategoryId: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub locationTextId: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub sortId: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub endPad: [u8;4],
}
