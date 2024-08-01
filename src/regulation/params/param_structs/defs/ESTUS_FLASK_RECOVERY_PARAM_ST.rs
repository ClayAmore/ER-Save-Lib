use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct ESTUS_FLASK_RECOVERY_PARAM_ST {
	pub host: u8,
	pub invadeOrb_None: u8,
	pub invadeOrb_Umbasa: u8,
	pub invadeOrb_Berserker: u8,
	pub invadeOrb_Sinners: u8,
	pub invadeSign_None: u8,
	pub invadeSign_Umbasa: u8,
	pub invadeSign_Berserker: u8,
	pub invadeSign_Sinners: u8,
	pub invadeRing_Sinners: u8,
	pub invadeRing_Rosalia: u8,
	pub invadeRing_Forest: u8,
	pub coopSign_None: u8,
	pub coopSign_Umbasa: u8,
	pub coopSign_Berserker: u8,
	pub coopSign_Sinners: u8,
	pub coopRing_RedHunter: u8,
	pub invadeRing_Anor: u8,
	pub paramReplaceRate: i16,
	pub paramReplaceId: i32,
	pub pad: [u8;8],
}
