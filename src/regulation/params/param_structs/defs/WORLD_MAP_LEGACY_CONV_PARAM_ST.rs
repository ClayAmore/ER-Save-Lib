use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WORLD_MAP_LEGACY_CONV_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub srcAreaNo: u8,
	pub srcGridXNo: u8,
	pub srcGridZNo: u8,
	pub pad1: [u8;1],
	pub srcPosX: f32,
	pub srcPosY: f32,
	pub srcPosZ: f32,
	pub dstAreaNo: u8,
	pub dstGridXNo: u8,
	pub dstGridZNo: u8,
	pub pad2: [u8;1],
	pub dstPosX: f32,
	pub dstPosY: f32,
	pub dstPosZ: f32,
	#[deku(bits = 7)]
	pub pad3: u8,
	#[deku(bits = 1)]
	pub isBasePoint: u8,
	#[deku(count = "11")]
	pub pad4: Vec<u8>,
}
