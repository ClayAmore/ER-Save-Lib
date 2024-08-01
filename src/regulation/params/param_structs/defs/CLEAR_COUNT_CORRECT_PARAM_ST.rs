use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CLEAR_COUNT_CORRECT_PARAM_ST {
	pub MaxHpRate: f32,
	pub MaxMpRate: f32,
	pub MaxStaminaRate: f32,
	pub PhysicsAttackRate: f32,
	pub SlashAttackRate: f32,
	pub BlowAttackRate: f32,
	pub ThrustAttackRate: f32,
	pub NeturalAttackRate: f32,
	pub MagicAttackRate: f32,
	pub FireAttackRate: f32,
	pub ThunderAttackRate: f32,
	pub DarkAttackRate: f32,
	pub PhysicsDefenseRate: f32,
	pub MagicDefenseRate: f32,
	pub FireDefenseRate: f32,
	pub ThunderDefenseRate: f32,
	pub DarkDefenseRate: f32,
	pub StaminaAttackRate: f32,
	pub SoulRate: f32,
	pub PoisionResistRate: f32,
	pub DiseaseResistRate: f32,
	pub BloodResistRate: f32,
	pub CurseResistRate: f32,
	pub FreezeResistRate: f32,
	pub BloodDamageRate: f32,
	pub SuperArmorDamageRate: f32,
	pub FreezeDamageRate: f32,
	pub SleepResistRate: f32,
	pub MadnessResistRate: f32,
	pub SleepDamageRate: f32,
	pub MadnessDamageRate: f32,
	pub pad1: [u8;4],
}
