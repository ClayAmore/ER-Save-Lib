use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct BONFIRE_WARP_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub eventflagId: i32,
	pub bonfireEntityId: i32,
	pub pad4: [u8;2],
	pub bonfireSubCategorySortId: i16,
	pub forbiddenIconId: i16,
	pub dispMinZoomStep: u8,
	pub selectMinZoomStep: u8,
	pub bonfireSubCategoryId: i32,
	pub clearedEventFlagId: i32,
	pub iconId: i16,
	#[deku(skip, cond = "version >= 11210015", bits = 6)]
	pub pad1_old: u8,
	#[deku(bits = 1)]
	pub dispMask01: u8,
	#[deku(bits = 1)]
	pub dispMask00: u8,
	#[deku(skip, cond = "version < 11210015", bits = 5)]
	pub pad1: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub dispMask02: u8,
	pub pad2: [u8;1],
	pub areaNo: u8,
	pub gridXNo: u8,
	pub gridZNo: u8,
	pub pad3: [u8;1],
	pub posX: f32,
	pub posY: f32,
	pub posZ: f32,
	pub textId1: i32,
	pub textEnableFlagId1: i32,
	pub textDisableFlagId1: i32,
	pub textId2: i32,
	pub textEnableFlagId2: i32,
	pub textDisableFlagId2: i32,
	pub textId3: i32,
	pub textEnableFlagId3: i32,
	pub textDisableFlagId3: i32,
	pub textId4: i32,
	pub textEnableFlagId4: i32,
	pub textDisableFlagId4: i32,
	pub textId5: i32,
	pub textEnableFlagId5: i32,
	pub textDisableFlagId5: i32,
	pub textId6: i32,
	pub textEnableFlagId6: i32,
	pub textDisableFlagId6: i32,
	pub textId7: i32,
	pub textEnableFlagId7: i32,
	pub textDisableFlagId7: i32,
	pub textId8: i32,
	pub textEnableFlagId8: i32,
	pub textDisableFlagId8: i32,
	pub textType1: u8,
	pub textType2: u8,
	pub textType3: u8,
	pub textType4: u8,
	pub textType5: u8,
	pub textType6: u8,
	pub textType7: u8,
	pub textType8: u8,
	pub noIgnitionSfxDmypolyId_0: i32,
	pub noIgnitionSfxId_0: i32,
	pub noIgnitionSfxDmypolyId_1: i32,
	pub noIgnitionSfxId_1: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textEnableFlag2Id1: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textEnableFlag2Id2: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textEnableFlag2Id3: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textEnableFlag2Id4: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textEnableFlag2Id5: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textEnableFlag2Id6: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textEnableFlag2Id7: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textEnableFlag2Id8: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textDisableFlag2Id1: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textDisableFlag2Id2: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textDisableFlag2Id3: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textDisableFlag2Id4: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textDisableFlag2Id5: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textDisableFlag2Id6: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textDisableFlag2Id7: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub textDisableFlag2Id8: i32,
	#[deku(skip, cond = "version < 10310059")]
	pub altIconId: i16,
	#[deku(skip, cond = "version < 10310059")]
	pub altForbiddenIconId: i16,
}
