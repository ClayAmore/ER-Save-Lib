use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct REINFORCE_PARAM_WEAPON_ST {
	pub physicsAtkRate: f32,
	pub magicAtkRate: f32,
	pub fireAtkRate: f32,
	pub thunderAtkRate: f32,
	pub staminaAtkRate: f32,
	pub saWeaponAtkRate: f32,
	pub saDurabilityRate: f32,
	pub correctStrengthRate: f32,
	pub correctAgilityRate: f32,
	pub correctMagicRate: f32,
	pub correctFaithRate: f32,
	pub physicsGuardCutRate: f32,
	pub magicGuardCutRate: f32,
	pub fireGuardCutRate: f32,
	pub thunderGuardCutRate: f32,
	pub poisonGuardResistRate: f32,
	pub diseaseGuardResistRate: f32,
	pub bloodGuardResistRate: f32,
	pub curseGuardResistRate: f32,
	pub staminaGuardDefRate: f32,
	pub spEffectId1: u8,
	pub spEffectId2: u8,
	pub spEffectId3: u8,
	pub residentSpEffectId1: u8,
	pub residentSpEffectId2: u8,
	pub residentSpEffectId3: u8,
	pub materialSetId: u8,
	pub maxReinforceLevel: u8,
	pub darkAtkRate: f32,
	pub darkGuardCutRate: f32,
	pub correctLuckRate: f32,
	pub freezeGuardDefRate: f32,
	pub reinforcePriceRate: f32,
	pub baseChangePriceRate: f32,
	pub enableGemRank: i8,
	pub pad2: [u8;3],
	pub sleepGuardDefRate: f32,
	pub madnessGuardDefRate: f32,
	pub baseAtkRate: f32,
}
