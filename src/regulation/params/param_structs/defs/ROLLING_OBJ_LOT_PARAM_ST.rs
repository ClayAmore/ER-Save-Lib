use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct ROLLING_OBJ_LOT_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub AssetId_0: i32,
	pub AssetId_1: i32,
	pub AssetId_2: i32,
	pub AssetId_3: i32,
	pub AssetId_4: i32,
	pub AssetId_5: i32,
	pub AssetId_6: i32,
	pub AssetId_7: i32,
	pub CreateWeight_0: u8,
	pub CreateWeight_1: u8,
	pub CreateWeight_2: u8,
	pub CreateWeight_3: u8,
	pub CreateWeight_4: u8,
	pub CreateWeight_5: u8,
	pub CreateWeight_6: u8,
	pub CreateWeight_7: u8,
	#[deku(count = "20")]
	pub Reserve_0: Vec<u8>,
}
