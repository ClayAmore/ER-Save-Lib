use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct REINFORCE_PARAM_PROTECTOR_ST {
	pub physicsDefRate: f32,
	pub magicDefRate: f32,
	pub fireDefRate: f32,
	pub thunderDefRate: f32,
	pub slashDefRate: f32,
	pub blowDefRate: f32,
	pub thrustDefRate: f32,
	pub resistPoisonRate: f32,
	pub resistDiseaseRate: f32,
	pub resistBloodRate: f32,
	pub resistCurseRate: f32,
	pub residentSpEffectId1: u8,
	pub residentSpEffectId2: u8,
	pub residentSpEffectId3: u8,
	pub materialSetId: u8,
	pub darkDefRate: f32,
	pub resistFreezeRate: f32,
	pub resistSleepRate: f32,
	pub resistMadnessRate: f32,
}
