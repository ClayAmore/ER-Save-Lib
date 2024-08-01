use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SOUND_AUTO_REVERB_SELECT_PARAM_ST {
	pub reverbType: i32,
	pub AreaNo: i32,
	pub IndoorOutdoor: i8,
	pub useDistNoA: i8,
	pub useDistNoB: i8,
	pub pad0: [u8;1],
	pub DistMinA: f32,
	pub DistMaxA: f32,
	pub DistMinB: f32,
	pub DistMaxB: f32,
	pub NoHitNumMin: i32,
}
