use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MAP_MIMICRY_ESTABLISHMENT_PARAM_ST {
	pub mimicryEstablishment0: f32,
	pub mimicryEstablishment1: f32,
	pub mimicryEstablishment2: f32,
	pub mimicryBeginSfxId0: i32,
	pub mimicrySfxId0: i32,
	pub mimicryEndSfxId0: i32,
	pub mimicryBeginSfxId1: i32,
	pub mimicrySfxId1: i32,
	pub mimicryEndSfxId1: i32,
	pub mimicryBeginSfxId2: i32,
	pub mimicrySfxId2: i32,
	pub mimicryEndSfxId2: i32,
	#[deku(count = "16")]
	pub pad1: Vec<u8>,
}
