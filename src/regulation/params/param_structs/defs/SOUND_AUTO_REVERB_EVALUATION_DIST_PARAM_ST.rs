use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST {
	pub NoHitDist: f32,
	pub isCollectNoHitPoint: u8,
	pub isCollectOutdoorPoint: u8,
	pub isCollectFloorPoint: u8,
	pub distValCalcType: u8,
	pub enableLifeTime: f32,
	pub maxDistRecordNum: i32,
	pub ignoreDistNumForMax: i32,
}
