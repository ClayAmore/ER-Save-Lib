use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct TOUGHNESS_PARAM_ST {
	pub correctionRate: f32,
	pub minToughness: i16,
	pub isNonEffectiveCorrectionForMin: u8,
	pub pad2: [u8;1],
	pub spEffectId: i32,
	pub proCorrectionRate: f32,
	pub unk1: f32,
	pub unk2: f32,
	pub pad1: [u8;8],
}
