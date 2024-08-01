use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct HIT_EFFECT_SFX_PARAM_ST {
	pub Slash_Normal: i32,
	pub Slash_S: i32,
	pub Slash_L: i32,
	pub Slash_Specific1: i32,
	pub Slash_Specific2: i32,
	pub Blow_Normal: i32,
	pub Blow_S: i32,
	pub Blow_L: i32,
	pub Blow_Specific1: i32,
	pub Blow_Specific2: i32,
	pub Thrust_Normal: i32,
	pub Thrust_S: i32,
	pub Thrust_L: i32,
	pub Thrust_Specific1: i32,
	pub Thrust_Specific2: i32,
	pub Neutral_Normal: i32,
	pub Neutral_S: i32,
	pub Neutral_L: i32,
	pub Neutral_Specific1: i32,
	pub Neutral_Specific2: i32,
}
