use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct NETWORK_AREA_PARAM_ST {
	pub cellSizeX: f32,
	pub cellSizeY: f32,
	pub cellSizeZ: f32,
	pub cellOffsetX: f32,
	pub cellOffsetY: f32,
	pub cellOffsetZ: f32,
	#[deku(bits = 1)]
	pub enableBreakInSearch: u8,
	#[deku(bits = 1)]
	pub enableRingSearch: u8,
	#[deku(bits = 1)]
	pub enableMultiPlay: u8,
	#[deku(bits = 1)]
	pub enableGhost: u8,
	#[deku(bits = 1)]
	pub enableBloodMessage: u8,
	#[deku(bits = 1)]
	pub enableBloodstain: u8,
	pub dummy: [u8;3],
}
