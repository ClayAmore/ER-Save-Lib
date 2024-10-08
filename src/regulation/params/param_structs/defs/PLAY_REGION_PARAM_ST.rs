use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct PLAY_REGION_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub matchAreaId: i32,
	pub multiPlayStartLimitEventFlagId: i32,
	pub otherDisableDistance: f32,
	pub pcPositionSaveLimitEventFlagId: i32,
	pub bossAreaId: i32,
	pub cultNpcWhiteGhostEntityId_byFree: i16,
	pub bMapGuradianRegion: u8,
	#[deku(bits = 1)]
	pub whiteSignLimitEventFlagId_2_targetFlagState: u8,
	#[deku(bits = 1)]
	pub breakInLimitEventFlagId_3_targetFlagState: u8,
	#[deku(bits = 1)]
	pub breakInLimitEventFlagId_2_targetFlagState: u8,
	#[deku(bits = 1)]
	pub redSignLimitEventFlagId_1_targetFlagState: u8,
	#[deku(bits = 1)]
	pub whiteSignLimitEventFlagId_1_targetFlagState: u8,
	#[deku(bits = 1)]
	pub breakInLimitEventFlagId_1_targetFlagState: u8,
	#[deku(bits = 1)]
	pub multiPlayStartLimitEventFlagId_targetFlagState: u8,
	#[deku(bits = 1)]
	pub bYellowCostumeRegion: u8,
	pub warpItemUsePermitBonfireId_1: i32,
	pub warpItemUsePermitBonfireId_2: i32,
	pub warpItemUsePermitBonfireId_3: i32,
	pub warpItemUsePermitBonfireId_4: i32,
	pub warpItemUsePermitBonfireId_5: i32,
	pub warpItemProhibitionEventFlagId_1: i32,
	pub warpItemProhibitionEventFlagId_2: i32,
	pub warpItemProhibitionEventFlagId_3: i32,
	pub warpItemProhibitionEventFlagId_4: i32,
	pub warpItemProhibitionEventFlagId_5: i32,
	#[deku(bits = 1)]
	pub redSignLimitEventFlagId_3_targetFlagState: u8,
	#[deku(bits = 1)]
	pub redSignLimitEventFlagId_2_targetFlagState: u8,
	#[deku(bits = 1)]
	pub whiteSignLimitEventFlagId_3_targetFlagState: u8,
	#[deku(bits = 1)]
	pub dispMask01: u8,
	#[deku(bits = 1)]
	pub dispMask00: u8,
	#[deku(bits = 1)]
	pub enableGhost: u8,
	#[deku(bits = 1)]
	pub enableBloodMessage: u8,
	#[deku(bits = 1)]
	pub enableBloodstain: u8,
	#[deku(skip, cond = "version >= 11210015", bits = 7)]
	pub pad1_old: u8,
	#[deku(bits = 1)]
	pub isAutoIntrudePoint: u8,
	#[deku(skip, cond = "version < 11210015", bits = 6)]
	pub pad1: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub unknown_0x45_1: u8,
	pub pad2: [u8;2],
	pub multiPlayHASHostLimitEventFlagId: i32,
	pub otherMaxDistance: f32,
	pub signPuddleOpenEventFlagId: i32,
	pub areaNo: u8,
	pub gridXNo: u8,
	pub gridZNo: u8,
	pub pad4: [u8;1],
	pub posX: f32,
	pub posY: f32,
	pub posZ: f32,
	pub breakInLimitEventFlagId_1: i32,
	pub whiteSignLimitEventFlagId_1: i32,
	pub matchAreaSignCreateLimitEventFlagId: i32,
	pub signAimId_1: i32,
	pub signAimId_2: i32,
	pub signAimId_3: i32,
	pub signAimId_4: i32,
	pub signAimId_5: i32,
	pub signAimId_6: i32,
	pub signAimId_7: i32,
	pub signAimId_8: i32,
	pub redSignLimitEventFlagId_1: i32,
	pub breakInLimitEventFlagId_2: i32,
	pub breakInLimitEventFlagId_3: i32,
	pub whiteSignLimitEventFlagId_2: i32,
	pub whiteSignLimitEventFlagId_3: i32,
	pub redSignLimitEventFlagId_2: i32,
	pub redSignLimitEventFlagId_3: i32,
	pub bossId_1: i32,
	pub bossId_2: i32,
	pub bossId_3: i32,
	pub bossId_4: i32,
	pub bossId_5: i32,
	pub bossId_6: i32,
	pub bossId_7: i32,
	pub bossId_8: i32,
	pub bossId_9: i32,
	pub bossId_10: i32,
	pub bossId_11: i32,
	pub bossId_12: i32,
	pub bossId_13: i32,
	pub bossId_14: i32,
	pub bossId_15: i32,
	pub bossId_16: i32,
	pub mapMenuUnlockEventId: i32,
	#[deku(count = "32")]
	pub pad5: Vec<u8>,
}
