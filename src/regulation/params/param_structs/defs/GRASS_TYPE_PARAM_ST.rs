use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct GRASS_TYPE_PARAM_ST {
	pub lodRange: i16,
	pub lod0ClusterType: u8,
	pub lod1ClusterType: u8,
	pub lod2ClusterType: u8,
	pub pad0: [u8;2],
	pub distributionType: u8,
	pub baseDensity: f32,
	#[deku(count = "16")]
	pub model0Name: Vec<u8>,
	#[deku(count = "32")]
	pub flatTextureName: Vec<u8>,
	#[deku(count = "32")]
	pub billboardTextureName: Vec<u8>,
	pub normalInfluence: u8,
	pub inclinationMax: u8,
	pub inclinationJitter: u8,
	pub scaleBaseMin: u8,
	pub scaleBaseMax: u8,
	pub scaleHeightMin: u8,
	pub scaleHeightMax: u8,
	pub colorShade1_r: u8,
	pub colorShade1_g: u8,
	pub colorShade1_b: u8,
	pub colorShade2_r: u8,
	pub colorShade2_g: u8,
	pub colorShade2_b: u8,
	pub flatSplitType: u8,
	pub flatBladeCount: u8,
	pub flatSlant: i8,
	pub flatRadius: f32,
	pub castShadow: u8,
	pub windAmplitude: u8,
	pub pad1: [u8;1],
	pub windCycle: u8,
	pub orientationAngle: f32,
	pub orientationRange: f32,
	pub spacing: f32,
	pub dithering: u8,
	pub pad: [u8;3],
	#[deku(count = "16")]
	pub simpleModelName: Vec<u8>,
	#[deku(count = "16")]
	pub model1Name: Vec<u8>,
}
