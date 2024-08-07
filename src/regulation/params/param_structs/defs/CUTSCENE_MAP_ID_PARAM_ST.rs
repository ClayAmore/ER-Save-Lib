use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CUTSCENE_MAP_ID_PARAM_ST {
	#[deku(bits = 6)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_Debug: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub PlayMapId: i32,
	pub RequireMapId0: i32,
	pub RequireMapId1: i32,
	pub RequireMapId2: i32,
	pub RefCamPosHitPartsID: i32,
	#[deku(skip, cond = "version >= 11210015", count = "12")]
	pub reserved_2_old: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x18: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub reserved_2: [u8;8],
	pub ClientDisableViewTimeForProgress: i16,
	pub reserved: [u8;2],
	pub HitParts_0: i32,
	pub HitParts_1: i32,
}
