use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct BEHAVIOR_PARAM_ST {
	pub variationId: i32,
	pub behaviorJudgeId: i32,
	pub ezStateBehaviorType_old: u8,
	pub refType: u8,
	pub pad2: [u8;2],
	pub refId: i32,
	pub consumeSA: f32,
	pub stamina: i32,
	pub consumeDurability: i32,
	pub category: u8,
	pub heroPoint: u8,
	pub pad1: [u8;2],
}
