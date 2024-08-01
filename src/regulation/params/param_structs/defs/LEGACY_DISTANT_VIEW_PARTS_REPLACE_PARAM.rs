use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM {
	pub TargetMapId: i32,
	pub TargetEventId: i32,
	pub SrcAssetId: i32,
	pub SrcAssetPartsNo: i32,
	pub DstAssetId: i32,
	pub DstAssetPartsNo: i32,
	pub SrcAssetIdRangeMin: i32,
	pub SrcAssetIdRangeMax: i32,
	pub DstAssetIdRangeMin: i32,
	pub DstAssetIdRangeMax: i32,
	pub LimitedMapRegionId0: i8,
	pub LimitedMapRegionId1: i8,
	pub LimitedMapRegionId2: i8,
	pub LimitedMapRegionId3: i8,
	pub reserve: [u8;4],
	pub LimitedMapRegionAssetId: i32,
	pub LimitedMapRegioAssetPartsNo: i32,
	pub LimitedMapRegioAssetIdRangeMin: i32,
	pub LimitedMapRegioAssetIdRangeMax: i32,
}
