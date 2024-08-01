use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WEATHER_ASSET_REPLACE_PARAM_ST {
	pub mapId: i32,
	pub TransitionSrcWeather: i16,
	pub padding0: [u8;2],
	pub isFireAsh: u8,
	pub padding1: [u8;3],
	pub reserved2: i32,
	pub AssetId0: i32,
	pub AssetId1: i32,
	pub AssetId2: i32,
	pub AssetId3: i32,
	pub AssetId4: i32,
	pub AssetId5: i32,
	pub AssetId6: i32,
	pub AssetId7: i32,
	pub reserved0: [u8;8],
	pub CreateAssetLimitId0: i8,
	pub CreateAssetLimitId1: i8,
	pub CreateAssetLimitId2: i8,
	pub CreateAssetLimitId3: i8,
	pub reserved1: [u8;4],
}
