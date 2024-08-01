use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct PHANTOM_PARAM_ST {
	pub edgeColorA: f32,
	pub frontColorA: f32,
	pub diffMulColorA: f32,
	pub specMulColorA: f32,
	pub lightColorA: f32,
	pub edgeColorR: u8,
	pub edgeColorG: u8,
	pub edgeColorB: u8,
	pub frontColorR: u8,
	pub frontColorG: u8,
	pub frontColorB: u8,
	pub diffMulColorR: u8,
	pub diffMulColorG: u8,
	pub diffMulColorB: u8,
	pub specMulColorR: u8,
	pub specMulColorG: u8,
	pub specMulColorB: u8,
	pub lightColorR: u8,
	pub lightColorG: u8,
	pub lightColorB: u8,
	pub reserve: [u8;1],
	pub alpha: f32,
	pub blendRate: f32,
	pub blendType: u8,
	pub isEdgeSubtract: u8,
	pub isFrontSubtract: u8,
	pub isNo2Pass: u8,
	pub edgePower: f32,
	pub glowScale: f32,
}
