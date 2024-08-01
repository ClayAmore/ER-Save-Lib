use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_GRAPHICS_CONFIG_PARAM_ST {
	pub m_textureFilterQuality: u8,
	pub m_aaQuality: u8,
	pub m_ssaoQuality: u8,
	pub m_dofQuality: u8,
	pub m_motionBlurQuality: u8,
	pub m_shadowQuality: u8,
	pub m_lightingQuality: u8,
	pub m_effectQuality: u8,
	pub m_decalQuality: u8,
	pub m_reflectionQuality: u8,
	pub m_waterQuality: u8,
	pub m_shaderQuality: u8,
	pub m_volumetricEffectQuality: u8,
	#[deku(skip, cond = "version >= 11210015")]
	pub m_dummy: [u8;3],
	#[deku(skip, cond = "version < 11210015")]
	pub m_RayTracingQuality: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub m_dummy1: [u8;2],
}
