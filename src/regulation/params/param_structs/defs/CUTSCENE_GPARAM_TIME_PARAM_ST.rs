use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CUTSCENE_GPARAM_TIME_PARAM_ST {
	#[deku(bits = 6)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_Debug: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub DstTimezone_Morning: u8,
	pub DstTimezone_Noon: u8,
	pub DstTimezone_AfterNoon: u8,
	pub DstTimezone_Evening: u8,
	pub DstTimezone_Night: u8,
	pub DstTimezone_DeepNightA: u8,
	pub DstTimezone_DeepNightB: u8,
	pub reserved: [u8;1],
	pub PostPlayIngameTime: f32,
}
