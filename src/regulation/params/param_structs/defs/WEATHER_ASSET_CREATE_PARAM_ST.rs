use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WEATHER_ASSET_CREATE_PARAM_ST {
	pub AssetId: i32,
	pub SlotNo: i32,
	pub CreateConditionType: u8,
	pub padding0: [u8;3],
	pub TransitionSrcWeather: i16,
	pub TransitionDstWeather: i16,
	pub ElapsedTimeCheckweather: i16,
	pub padding1: [u8;2],
	pub ElapsedTime: f32,
	pub CreateDelayTime: f32,
	pub CreateProbability: i32,
	pub LifeTime: f32,
	pub FadeTime: f32,
	pub EnableCreateTimeMin: f32,
	pub EnableCreateTimeMax: f32,
	pub CreatePoint0: i16,
	pub CreatePoint1: i16,
	pub CreatePoint2: i16,
	pub CreatePoint3: i16,
	pub CreateAssetLimitId0: i8,
	pub CreateAssetLimitId1: i8,
	pub CreateAssetLimitId2: i8,
	pub CreateAssetLimitId3: i8,
	pub Reserved2: [u8;4],
}
