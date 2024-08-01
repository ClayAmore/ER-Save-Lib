use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct ENEMY_COMMON_PARAM_ST {
	pub reserved0: [u8;8],
	pub soundTargetTryApproachTime: i32,
	pub searchTargetTryApproachTime: i32,
	pub memoryTargetTryApproachTime: i32,
	#[deku(count = "40")]
	pub reserved5: Vec<u8>,
	pub activateChrByTime_PhantomId: i32,
	pub findUnfavorableFailedPointDist: f32,
	pub findUnfavorableFailedPointHeight: f32,
	#[deku(count = "184")]
	pub reserved18: Vec<u8>,
}
