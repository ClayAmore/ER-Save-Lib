use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct NPC_AI_ACTION_PARAM_ST {
	pub moveDir: u8,
	pub key1: u8,
	pub key2: u8,
	pub key3: u8,
	pub bMoveDirHold: u8,
	pub bKeyHold1: u8,
	pub bKeyHold2: u8,
	pub bKeyHold3: u8,
	pub gestureId: i32,
	pub bLifeEndSuccess: u8,
	pub pad1: [u8;3],
}
