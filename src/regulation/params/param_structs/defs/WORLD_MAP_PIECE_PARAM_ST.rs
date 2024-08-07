use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WORLD_MAP_PIECE_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub openEventFlagId: i32,
	pub openTravelAreaLeft: f32,
	pub openTravelAreaRight: f32,
	pub openTravelAreaTop: f32,
	pub openTravelAreaBottom: f32,
	pub acquisitionEventFlagId: i32,
	pub acquisitionEventScale: f32,
	pub acquisitionEventCenterX: f32,
	pub acquisitionEventCenterY: f32,
	pub acquisitionEventResScale: f32,
	pub acquisitionEventResOffsetX: f32,
	pub acquisitionEventResOffsetY: f32,
	#[deku(count = "12")]
	pub pad: Vec<u8>,
}
