use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SP_EFFECT_VFX_PARAM_ST {
	pub midstSfxId: i32,
	pub midstSeId: i32,
	pub initSfxId: i32,
	pub initSeId: i32,
	pub finishSfxId: i32,
	pub finishSeId: i32,
	pub camouflageBeginDist: f32,
	pub camouflageEndDist: f32,
	pub transformProtectorId: i32,
	pub midstDmyId: i16,
	pub initDmyId: i16,
	pub finishDmyId: i16,
	pub effectType: u8,
	pub soulParamIdForWepEnchant: u8,
	pub playCategory: u8,
	pub playPriority: u8,
	#[deku(bits = 1)]
	pub isFullBodyTransformProtectorId: u8,
	#[deku(bits = 1)]
	pub halfCamouflage: u8,
	#[deku(bits = 1)]
	pub isHideFootEffect_forCamouflage: u8,
	#[deku(bits = 1)]
	pub invisibleAtFriendCamouflage: u8,
	#[deku(bits = 1)]
	pub useCamouflage: u8,
	#[deku(bits = 1)]
	pub effectInvisibleAtCamouflage: u8,
	#[deku(bits = 1)]
	pub existEffectForSoul: u8,
	#[deku(bits = 1)]
	pub existEffectForLarge: u8,
	#[deku(skip, cond = "version >= 11210015", bits = 1)]
	pub pad_1: u8,
	#[deku(bits = 1)]
	pub isUseOffsetEnchantSfxSize: u8,
	#[deku(bits = 1)]
	pub isVisibleDeadChr: u8,
	#[deku(bits = 1)]
	pub isFinishFullbody: u8,
	#[deku(bits = 1)]
	pub isInitFullbody: u8,
	#[deku(bits = 1)]
	pub isMidstFullbody: u8,
	#[deku(bits = 1)]
	pub isSilence: u8,
	#[deku(bits = 1)]
	pub isInvisibleWeapon: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub unknown_0x2f_7: u8,
	pub decalId1: i32,
	pub decalId2: i32,
	pub footEffectPriority: u8,
	pub footEffectOffset: u8,
	pub traceSfxIdOffsetType: u8,
	pub forceDeceasedType: u8,
	pub enchantStartDmyId_0: i32,
	pub enchantEndDmyId_0: i32,
	pub enchantStartDmyId_1: i32,
	pub enchantEndDmyId_1: i32,
	pub enchantStartDmyId_2: i32,
	pub enchantEndDmyId_2: i32,
	pub enchantStartDmyId_3: i32,
	pub enchantEndDmyId_3: i32,
	pub enchantStartDmyId_4: i32,
	pub enchantEndDmyId_4: i32,
	pub enchantStartDmyId_5: i32,
	pub enchantEndDmyId_5: i32,
	pub enchantStartDmyId_6: i32,
	pub enchantEndDmyId_6: i32,
	pub enchantStartDmyId_7: i32,
	pub enchantEndDmyId_7: i32,
	pub SfxIdOffsetType: u8,
	pub phantomParamOverwriteType: u8,
	pub camouflageMinAlpha: u8,
	pub wetAspectType: u8,
	pub phantomParamOverwriteId: i32,
	pub materialParamId: i32,
	pub materialParamInitValue: f32,
	pub materialParamTargetValue: f32,
	pub materialParamFadeTime: f32,
	pub footDecalMaterialOffsetOverwriteId: i16,
	#[deku(skip, cond = "version >= 11210015", count = "14")]
	pub pad_old: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x96: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x97: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x98: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x99: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x9a: u8,
	#[deku(skip, cond = "version < 11210015", count = "9")]
	pub pad: Vec<u8>,
}
