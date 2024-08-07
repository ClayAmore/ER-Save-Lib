use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct HIT_MTRL_PARAM_ST {
	pub aiVolumeRate: f32,
	pub spEffectIdOnHit0: i32,
	pub spEffectIdOnHit1: i32,
	#[deku(bits = 1)]
	pub disableFallDamage: u8,
	#[deku(bits = 2)]
	pub floorHeightType: u8,
	#[deku(bits = 2)]
	pub footEffectDirType: u8,
	#[deku(bits = 2)]
	pub footEffectHeightType: u8,
	#[deku(bits = 1)]
	pub isHardnessForSoundReverb: u8,
	pub hardnessType: u8,
	pub pad2: [u8;6],
	pub spEffectIdOnHit0_ClearCount_2: i32,
	pub spEffectIdOnHit0_ClearCount_3: i32,
	pub spEffectIdOnHit0_ClearCount_4: i32,
	pub spEffectIdOnHit0_ClearCount_5: i32,
	pub spEffectIdOnHit0_ClearCount_6: i32,
	pub spEffectIdOnHit0_ClearCount_7: i32,
	pub spEffectIdOnHit0_ClearCount_8: i32,
	pub spEffectIdOnHit1_ClearCount_2: i32,
	pub spEffectIdOnHit1_ClearCount_3: i32,
	pub spEffectIdOnHit1_ClearCount_4: i32,
	pub spEffectIdOnHit1_ClearCount_5: i32,
	pub spEffectIdOnHit1_ClearCount_6: i32,
	pub spEffectIdOnHit1_ClearCount_7: i32,
	pub spEffectIdOnHit1_ClearCount_8: i32,
	pub replaceMateiralId_Rain: i16,
	pub pad4: [u8;2],
	pub spEffectId_forWet00: i32,
	pub spEffectId_forWet01: i32,
	pub spEffectId_forWet02: i32,
	pub spEffectId_forWet03: i32,
	pub spEffectId_forWet04: i32,
}
