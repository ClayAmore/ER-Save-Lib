use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct LOCK_CAM_PARAM_ST {
	pub camDistTarget: f32,
	pub rotRangeMinX: f32,
	pub lockRotXShiftRatio: f32,
	pub chrOrgOffset_Y: f32,
	pub chrLockRangeMaxRadius: f32,
	pub camFovY: f32,
	pub chrLockRangeMaxRadius_forD: f32,
	pub chrLockRangeMaxRadius_forPD: f32,
	pub closeMaxHeight: f32,
	pub closeMinHeight: f32,
	pub closeAngRange: f32,
	pub closeMaxRadius: f32,
	pub closeMaxRadius_forD: f32,
	pub closeMaxRadius_forPD: f32,
	pub bulletMaxRadius: f32,
	pub bulletMaxRadius_forD: f32,
	pub bulletMaxRadius_forPD: f32,
	pub bulletAngRange: f32,
	pub lockTgtKeepTime: f32,
	pub chrTransChaseRateForNormal: f32,
	#[deku(count = "48")]
	pub pad: Vec<u8>,
}
