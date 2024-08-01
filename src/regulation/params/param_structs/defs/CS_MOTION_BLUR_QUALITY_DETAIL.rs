use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_MOTION_BLUR_QUALITY_DETAIL {
	pub enabled: u8,
	pub ombEnabled: u8,
	pub forceScaleVelocityBuffer: u8,
	pub cheapFilterMode: u8,
	pub sampleCountBias: i32,
	pub recurrenceCountBias: i32,
	pub blurMaxLengthScale: f32,
}
