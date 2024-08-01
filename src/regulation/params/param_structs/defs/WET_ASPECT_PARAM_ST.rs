use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WET_ASPECT_PARAM_ST {
	pub baseColorR: u8,
	pub baseColorG: u8,
	pub baseColorB: u8,
	pub reserve_0: [u8;1],
	pub baseColorA: f32,
	pub metallic: u8,
	pub reserve_1: [u8;1],
	pub reserve_2: [u8;1],
	pub reserve_3: [u8;1],
	pub metallicRate: f32,
	pub shininessRate: f32,
	pub shininess: u8,
	#[deku(count = "11")]
	pub reserve_4: Vec<u8>,
}
