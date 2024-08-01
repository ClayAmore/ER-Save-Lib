use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL {
	pub fogEnabled: u8,
	pub fogShadowEnabled: u8,
	pub dmy: [u8;2],
	pub fogShadowSampleCountBias: i32,
	pub fogLocalLightDistScale: f32,
	pub fogVolueSizeScaler: i32,
	pub fogVolueSizeDivider: i32,
	pub fogVolumeDepthScaler: i32,
	pub fogVolumeDepthDivider: i32,
	pub fogVolumeEnabled: u8,
	pub fogVolumeUpScaleType: u8,
	pub fogVolumeEdgeCorrectionLevel: u8,
	pub fogVolumeRayMarcingSampleCountOffset: i8,
	pub fogVolumeShadowEnabled: u8,
	pub fogVolumeForceShadowing: u8,
	pub fogVolumeResolution: u8,
	pub pad2: [u8;1],
}
