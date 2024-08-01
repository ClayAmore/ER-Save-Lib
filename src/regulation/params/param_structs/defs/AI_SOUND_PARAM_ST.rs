use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct AI_SOUND_PARAM_ST {
	pub radius: f32,
	pub lifeFrame: f32,
	pub bSpEffectEnable: u8,
	pub r#type: u8,
	#[deku(bits = 1)]
	pub opposeTarget: u8,
	#[deku(bits = 1)]
	pub friendlyTarget: u8,
	#[deku(bits = 1)]
	pub selfTarget: u8,
	#[deku(bits = 1)]
	pub disableOnTargetPCompany: u8,
	pub rank: u8,
	pub forgetTime: f32,
	pub priority: i32,
	pub soundBehaviorId: i32,
	pub aiSoundLevel: u8,
	pub replaningState: u8,
	pub pad1: [u8;6],
}
