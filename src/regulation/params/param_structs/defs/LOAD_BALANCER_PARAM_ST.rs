use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct LOAD_BALANCER_PARAM_ST {
	pub lowerFpsThreshold: f32,
	pub upperFpsThreshold: f32,
	pub lowerFpsContinousCount: i32,
	pub upperFpsContinousCount: i32,
	pub downAfterChangeSleep: i32,
	pub upAfterChangeSleep: i32,
	pub postProcessLightShaft: u8,
	pub postProcessBloom: u8,
	pub postProcessGlow: u8,
	pub postProcessAA: u8,
	pub postProcessSSAO: u8,
	pub postProcessDOF: u8,
	pub postProcessMotionBlur: u8,
	pub postProcessMotionBlurIteration: u8,
	pub reserve0: [u8;1],
	pub shadowBlur: u8,
	pub sfxParticleHalf: u8,
	pub sfxReflection: u8,
	pub sfxWaterInteraction: u8,
	pub sfxGlow: u8,
	pub sfxDistortion: u8,
	pub sftSoftSprite: u8,
	pub sfxLightShaft: u8,
	pub sfxScaleRenderDistanceScale: u8,
	pub dynamicResolution: u8,
	pub shadowCascade0ResolutionHalf: u8,
	pub shadowCascade1ResolutionHalf: u8,
	pub chrWetDisablePlayer: u8,
	pub chrWetDisableRemotePlayer: u8,
	pub chrWetDisableEnemy: u8,
	pub dynamicResolutionPercentageMin: u8,
	pub dynamicResolutionPercentageMax: u8,
	#[deku(count = "30")]
	pub reserve1: Vec<u8>,
}
