use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct BUDDY_STONE_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub talkChrEntityId: i32,
	pub eliminateTargetEntityId: i32,
	pub summonedEventFlagId: i32,
	#[deku(bits = 7)]
	pub pad1: u8,
	#[deku(bits = 1)]
	pub isSpecial: u8,
	pub pad2: [u8;3],
	pub buddyId: i32,
	pub dopingSpEffectId: i32,
	pub activateRange: i16,
	pub overwriteReturnRange: i16,
	pub overwriteActivateRegionEntityId: i32,
	pub warnRegionEntityId: i32,
	#[deku(count = "24")]
	pub pad3: Vec<u8>,
}
