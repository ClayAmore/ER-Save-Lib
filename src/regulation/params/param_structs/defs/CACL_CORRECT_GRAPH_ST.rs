use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CACL_CORRECT_GRAPH_ST {
	pub stageMaxVal0: f32,
	pub stageMaxVal1: f32,
	pub stageMaxVal2: f32,
	pub stageMaxVal3: f32,
	pub stageMaxVal4: f32,
	pub stageMaxGrowVal0: f32,
	pub stageMaxGrowVal1: f32,
	pub stageMaxGrowVal2: f32,
	pub stageMaxGrowVal3: f32,
	pub stageMaxGrowVal4: f32,
	pub adjPt_maxGrowVal0: f32,
	pub adjPt_maxGrowVal1: f32,
	pub adjPt_maxGrowVal2: f32,
	pub adjPt_maxGrowVal3: f32,
	pub adjPt_maxGrowVal4: f32,
	pub init_inclination_soul: f32,
	pub adjustment_value: f32,
	pub boundry_inclination_soul: f32,
	pub boundry_value: f32,
	pub pad: [u8;4],
}
