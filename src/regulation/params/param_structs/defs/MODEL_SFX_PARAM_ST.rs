use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MODEL_SFX_PARAM_ST {
	pub sfxId_0: i32,
	pub dmypolyId_0: i32,
	pub reserve_0: [u8;8],
	pub sfxId_1: i32,
	pub dmypolyId_1: i32,
	pub reserve_1: [u8;8],
	pub sfxId_2: i32,
	pub dmypolyId_2: i32,
	pub reserve_2: [u8;8],
	pub sfxId_3: i32,
	pub dmypolyId_3: i32,
	pub reserve_3: [u8;8],
	pub sfxId_4: i32,
	pub dmypolyId_4: i32,
	pub reserve_4: [u8;8],
	pub sfxId_5: i32,
	pub dmypolyId_5: i32,
	pub reserve_5: [u8;8],
	pub sfxId_6: i32,
	pub dmypolyId_6: i32,
	pub reserve_6: [u8;8],
	pub sfxId_7: i32,
	pub dmypolyId_7: i32,
	pub reserve_7: [u8;8],
}
