use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct OBJECT_MATERIAL_SFX_PARAM_ST {
	pub sfxId_00: i32,
	pub sfxId_01: i32,
	pub sfxId_02: i32,
	pub sfxId_03: i32,
	pub sfxId_04: i32,
	pub sfxId_05: i32,
	pub sfxId_06: i32,
	pub sfxId_07: i32,
	pub sfxId_08: i32,
	pub sfxId_09: i32,
	pub sfxId_10: i32,
	pub sfxId_11: i32,
	pub sfxId_12: i32,
	pub sfxId_13: i32,
	pub sfxId_14: i32,
	pub sfxId_15: i32,
	pub sfxId_16: i32,
	pub sfxId_17: i32,
	pub sfxId_18: i32,
	pub sfxId_19: i32,
	pub sfxId_20: i32,
	pub sfxId_21: i32,
	pub sfxId_22: i32,
	pub sfxId_23: i32,
	pub sfxId_24: i32,
	pub sfxId_25: i32,
	pub sfxId_26: i32,
	pub sfxId_27: i32,
	pub sfxId_28: i32,
	pub sfxId_29: i32,
	pub sfxId_30: i32,
	pub sfxId_31: i32,
}
