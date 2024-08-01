use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct BUDDY_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub triggerSpEffectId: i32,
	pub npcParamId: i32,
	pub npcThinkParamId: i32,
	pub npcParamId_ridden: i32,
	pub npcThinkParamId_ridden: i32,
	pub x_offset: f32,
	pub z_offset: f32,
	pub y_angle: f32,
	pub appearOnAroundSekihi: u8,
	pub disablePCTargetShare: u8,
	pub pcFollowType: u8,
	pub Reserve: [u8;1],
	pub dopingSpEffect_lv0: i32,
	pub dopingSpEffect_lv1: i32,
	pub dopingSpEffect_lv2: i32,
	pub dopingSpEffect_lv3: i32,
	pub dopingSpEffect_lv4: i32,
	pub dopingSpEffect_lv5: i32,
	pub dopingSpEffect_lv6: i32,
	pub dopingSpEffect_lv7: i32,
	pub dopingSpEffect_lv8: i32,
	pub dopingSpEffect_lv9: i32,
	pub dopingSpEffect_lv10: i32,
	pub npcPlayerInitParamId: i32,
	pub generateAnimId: i32,
	#[deku(skip, cond = "version >= 10801000")]
	pub Reserve2: [u8;4],
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x5c: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x60: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x64: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x68: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x6c: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x70: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x74: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x78: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x7c: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x80: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x84: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x88: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x8c: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x90: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x94: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x98: i32,
	#[deku(skip, cond = "version < 10801000")]
	pub unknown_0x9c: i32,
}
