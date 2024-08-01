use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_REFLECTION_QUALITY_DETAIL {
	pub enabled: u8,
	pub localLightEnabled: u8,
	pub localLightForceEnabled: u8,
	pub dmy: [u8;1],
	pub resolutionDivider: i32,
	pub ssrEnabled: u8,
	pub ssrGaussianBlurEnabled: u8,
	pub dmy2: [u8;2],
	pub ssrDepthRejectThresholdScale: f32,
	pub ssrRayTraceStepScale: f32,
	pub ssrFadeToViewerBias: f32,
	pub ssrFresnelRejectBias: f32,
}
