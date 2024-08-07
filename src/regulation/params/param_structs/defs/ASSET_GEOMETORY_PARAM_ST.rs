use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct ASSET_GEOMETORY_PARAM_ST {
	pub soundBankId: i32,
	pub soundBreakSEId: i32,
	pub refDrawParamId: i32,
	pub hitCreateType: i8,
	pub behaviorType: u8,
	pub collisionType: u8,
	pub rainBlockingType: u8,
	pub hp: i16,
	pub defense: i16,
	pub breakStopTime: f32,
	pub breakSfxId: i32,
	pub breakSfxCpId: i32,
	pub breakLandingSfxId: i32,
	pub breakBulletBehaviorId: i32,
	pub breakBulletCpId: i32,
	pub FragmentInvisibleWaitTime: f32,
	pub FragmentInvisibleTime: f32,
	pub BreakAiSoundID: i32,
	pub breakItemLotType: i8,
	pub animBreakIdMax: u8,
	pub breakBulletAttributeDamageType: i8,
	#[deku(bits = 1)]
	pub isAttackBacklash: u8,
	#[deku(bits = 1)]
	pub isDamageCover: u8,
	#[deku(bits = 1)]
	pub isAnimBreak: u8,
	#[deku(bits = 1)]
	pub isDisableBreakForFirstAppear: u8,
	#[deku(bits = 1)]
	pub isBreak_ByChrRide: u8,
	#[deku(bits = 1)]
	pub isBreakByEnemyCollide: u8,
	#[deku(bits = 1)]
	pub isBreakByPlayerCollide: u8,
	#[deku(bits = 1)]
	pub isBurn: u8,
	#[deku(bits = 1)]
	pub isAnimPauseOnRemoPlay: u8,
	#[deku(bits = 1)]
	pub isSkydomeFlag: u8,
	#[deku(bits = 1)]
	pub isMoveObj: u8,
	#[deku(bits = 1)]
	pub isLadder: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub unknown_0x3b_7: u8,
	#[deku(skip, cond = "version >= 11210015", bits = 1)]
	pub Reserve_2: u8,
	#[deku(bits = 1)]
	pub isBreakByHugeenemyCollide: u8,
	#[deku(bits = 1)]
	pub isBreakOnPickUp: u8,
	#[deku(bits = 1)]
	pub isEnableRepick: u8,
	pub navimeshFlag: u8,
	pub burnBulletInterval: i16,
	pub clothUpdateDist: f32,
	pub lifeTime_forRuntimeCreate: f32,
	pub contactSeId: i32,
	pub repickAnimIdOffset: i32,
	pub windEffectRate_0: f32,
	pub windEffectRate_1: f32,
	pub windEffectType_0: u8,
	pub windEffectType_1: u8,
	pub overrideMaterialId: i16,
	pub autoCreateOffsetHeight: f32,
	pub burnTime: f32,
	pub burnBraekRate: f32,
	pub burnSfxId: i32,
	pub burnSfxId_1: i32,
	pub burnSfxId_2: i32,
	pub burnSfxId_3: i32,
	pub burnSfxDelayTimeMin: f32,
	pub burnSfxDelayTimeMin_1: f32,
	pub burnSfxDelayTimeMin_2: f32,
	pub burnSfxDelayTimeMin_3: f32,
	pub burnSfxDelayTimeMax: f32,
	pub burnSfxDelayTimeMax_1: f32,
	pub burnSfxDelayTimeMax_2: f32,
	pub burnSfxDelayTimeMax_3: f32,
	pub burnBulletBehaviorId: i32,
	pub burnBulletBehaviorId_1: i32,
	pub burnBulletBehaviorId_2: i32,
	pub burnBulletBehaviorId_3: i32,
	pub burnBulletDelayTime: f32,
	pub paintDecalTargetTextureSize: i16,
	pub navimeshFlag_after: u8,
	pub camNearBehaviorType: i8,
	pub breakItemLotParamId: i32,
	pub pickUpActionButtonParamId: i32,
	pub pickUpItemLotParamId: i32,
	pub autoDrawGroupBackFaceCheck: u8,
	pub autoDrawGroupDepthWrite: u8,
	pub autoDrawGroupShadowTest: u8,
	pub debug_isHeightCheckEnable: u8,
	pub hitCarverCancelAreaFlag: u8,
	pub assetNavimeshNoCombine: u8,
	pub navimeshFlagApply: u8,
	pub navimeshFlagApply_after: u8,
	pub autoDrawGroupPassPixelNum: f32,
	pub pickUpReplacementEventFlag: i32,
	pub pickUpReplacementAnimIdOffset: i32,
	pub pickUpReplacementActionButtonParamId: i32,
	pub pickUpReplacementItemLotParamId: i32,
	pub slidingBulletHitType: u8,
	pub isBushesForDamage: u8,
	pub penetrationBulletType: u8,
	pub unkR3: u8,
	pub unkR4: f32,
	pub soundBreakSECpId: i32,
	pub debug_HeightCheckCapacityMin: f32,
	pub debug_HeightCheckCapacityMax: f32,
	pub repickActionButtonParamId: i32,
	pub repickItemLotParamId: i32,
	pub repickReplacementAnimIdOffset: i32,
	pub repickReplacementActionButtonParamId: i32,
	pub repickReplacementItemLotParamId: i32,
	pub noGenerateCarver: u8,
	pub noHitHugeAfterBreak: u8,
	#[deku(bits = 2)]
	pub unkR1: u8,
	#[deku(bits = 1)]
	pub isEnableSignPostBreak: u8,
	#[deku(bits = 1)]
	pub isEnableSignPreBreak: u8,
	#[deku(bits = 1)]
	pub isDisableBulletHitSfx: u8,
	#[deku(bits = 1)]
	pub isCreateMultiPlayOnly: u8,
	#[deku(bits = 1)]
	pub isHiddenOnRepick: u8,
	#[deku(bits = 1)]
	pub isEnabledBreakSync: u8,
	pub generateMultiForbiddenRegion: u8,
	pub residentSeId0: i32,
	pub residentSeId1: i32,
	pub residentSeId2: i32,
	pub residentSeId3: i32,
	pub residentSeDmypolyId0: i16,
	pub residentSeDmypolyId1: i16,
	pub residentSeDmypolyId2: i16,
	pub residentSeDmypolyId3: i16,
	pub excludeActivateRatio_Xboxone_Grid: u8,
	pub excludeActivateRatio_Xboxone_Legacy: u8,
	pub excludeActivateRatio_PS4_Grid: u8,
	pub excludeActivateRatio_PS4_Legacy: u8,
	#[deku(skip, cond = "version >= 11210015", count = "32")]
	pub Reserve_0_old: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x120: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x121: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x122: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x123: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x124: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x125: u8,
	#[deku(skip, cond = "version < 11210015", count = "26")]
	pub Reserve_0: Vec<u8>,
}
