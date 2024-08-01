use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WEATHER_PARAM_ST {
	pub SfxId: i32,
	pub WindSfxId: i32,
	pub GroundHitSfxId: i32,
	pub SoundId: i32,
	pub WetTime: f32,
	pub GparamId: i32,
	pub NextLotIngameSecondsMin: i32,
	pub NextLotIngameSecondsMax: i32,
	pub WetSpEffectId00: i32,
	pub WetSpEffectId01: i32,
	pub WetSpEffectId02: i32,
	pub WetSpEffectId03: i32,
	pub WetSpEffectId04: i32,
	pub SfxIdInoor: i32,
	pub SfxIdOutdoor: i32,
	pub aiSightRate: f32,
	pub DistViewWeatherGparamOverrideWeight: f32,
}
