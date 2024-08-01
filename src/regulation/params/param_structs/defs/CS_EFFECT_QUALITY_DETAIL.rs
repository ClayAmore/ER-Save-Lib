use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_EFFECT_QUALITY_DETAIL {
	pub softParticleEnabled: u8,
	pub glowEnabled: u8,
	pub distortionEnable: u8,
	pub cs_upScaleEnabledType: u8,
	pub fNumOnceEmitsScale: f32,
	pub fEmitSpanScale: f32,
	pub fLodDistance1Scale: f32,
	pub fLodDistance2Scale: f32,
	pub fLodDistance3Scale: f32,
	pub fLodDistance4Scale: f32,
	pub fScaleRenderDistanceScale: f32,
	pub dmy: [u8;4],
}
