use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MULTI_SOUL_BONUS_RATE_PARAM_ST {
	pub host: f32,
	pub WhiteGhost_None: f32,
	pub WhiteGhost_Umbasa: f32,
	pub WhiteGhost_Berserker: f32,
	pub BlackGhost_None_Sign: f32,
	pub BlackGhost_Umbasa_Sign: f32,
	pub BlackGhost_Berserker_Sign: f32,
	pub BlackGhost_None_Invade: f32,
	pub BlackGhost_Umbasa_Invade: f32,
	pub BlackGhost_Berserker_Invade: f32,
	pub RedHunter1: f32,
	pub RedHunter2: f32,
	pub GuardianOfForest: f32,
	pub GuardianOfAnor: f32,
	pub BattleRoyal: f32,
	pub YellowMonk: f32,
	#[deku(count = "64")]
	pub pad1: Vec<u8>,
}
