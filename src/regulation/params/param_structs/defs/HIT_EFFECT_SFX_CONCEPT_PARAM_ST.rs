use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct HIT_EFFECT_SFX_CONCEPT_PARAM_ST {
	pub atkIron_1: i16,
	pub atkIron_2: i16,
	pub atkLeather_1: i16,
	pub atkLeather_2: i16,
	pub atkWood_1: i16,
	pub atkWood_2: i16,
	pub atkBody_1: i16,
	pub atkBody_2: i16,
	pub atkStone_1: i16,
	pub atkStone_2: i16,
	pub pad: [u8;4],
	pub atkNone_1: i16,
	pub atkNone_2: i16,
	#[deku(count = "52")]
	pub reserve: Vec<u8>,
}
