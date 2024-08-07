use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CHR_ACTIVATE_CONDITION_PARAM_ST {
	#[deku(bits = 1)]
	pub weatherStorm: u8,
	#[deku(bits = 1)]
	pub weatherHeavyRain: u8,
	#[deku(bits = 1)]
	pub weatherRain: u8,
	#[deku(bits = 1)]
	pub weatherCloudy: u8,
	#[deku(bits = 1)]
	pub weatherWeakCloudy: u8,
	#[deku(bits = 1)]
	pub weatherClearSky: u8,
	#[deku(bits = 1)]
	pub weatherSunny: u8,
	#[deku(bits = 1)]
	pub weatherSandStorm: u8,
	#[deku(bits = 1)]
	pub weatherHeavyFogRain: u8,
	#[deku(bits = 1)]
	pub weatherHeavyFog: u8,
	#[deku(bits = 1)]
	pub weatherFog: u8,
	#[deku(bits = 1)]
	pub weatherHeavySnow: u8,
	#[deku(bits = 1)]
	pub weatherSnow: u8,
	#[deku(bits = 1)]
	pub weatherStormForBattle: u8,
	#[deku(bits = 2)]
	pub pad1: u8,
	pub timeStartHour: u8,
	pub timeStartMin: u8,
	pub timeEndHour: u8,
	pub timeEndMin: u8,
	pub pad2: [u8;2],
}
