use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MAP_GRID_CREATE_HEIGHT_LIMIT_DETAIL_INFO_PARAM_ST {
	pub mapId: i32,
	pub unknown_0x4: i32,
	pub unknown_0x8: i32,
	pub unknown_0xc: i32,
	pub unknown_0x10: i32,
	pub unknown_0x14: i32,
	pub unknown_0x18: i32,
	pub unknown_0x1c: i32,
	pub unknown_0x20: i32,
	pub unknown_0x24: i32,
	pub unknown_0x28: i32,
	pub unknown_0x2c: u8,
	pub unknown_0x2d: u8,
	pub unknown_0x2e: u8,
	pub unknown_0x2f: u8,
	pub unknown_0x30: u8,
	pub unknown_0x31: u8,
	pub unknown_0x32: i16,
	pub unknown_0x34: i32,
	pub unknown_0x38: i32,
	pub unknown_0x3c: i32,
}
