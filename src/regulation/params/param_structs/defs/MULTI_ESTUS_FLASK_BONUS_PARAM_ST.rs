use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MULTI_ESTUS_FLASK_BONUS_PARAM_ST {
	pub host: u8,
	pub WhiteGhost_None: u8,
	pub WhiteGhost_Umbasa: u8,
	pub WhiteGhost_Berserker: u8,
	pub BlackGhost_None_Sign: u8,
	pub BlackGhost_Umbasa_Sign: u8,
	pub BlackGhost_Berserker_Sign: u8,
	pub BlackGhost_None_Invade: u8,
	pub BlackGhost_Umbasa_Invade: u8,
	pub BlackGhost_Berserker_Invade: u8,
	pub RedHunter1: u8,
	pub RedHunter2: u8,
	pub GuardianOfForest: u8,
	pub GuardianOfAnor: u8,
	pub BattleRoyal: u8,
	pub YellowMonk: u8,
	#[deku(count = "48")]
	pub pad1: Vec<u8>,
}
