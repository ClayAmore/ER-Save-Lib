use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct EQUIP_PARAM_GOODS_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub refId_default: i32,
	pub sfxVariationId: i32,
	pub weight: f32,
	pub basicPrice: i32,
	pub sellValue: i32,
	pub behaviorId: i32,
	pub replaceItemId: i32,
	pub sortId: i32,
	pub appearanceReplaceItemId: i32,
	pub yesNoDialogMessageId: i32,
	pub useEnableSpEffectType: i16,
	pub potGroupId: i8,
	pub pad: [u8;1],
	pub iconId: i16,
	pub modelId: i16,
	pub shopLv: i16,
	pub compTrophySedId: i16,
	pub trophySeqId: i16,
	pub maxNum: i16,
	pub consumeHeroPoint: u8,
	pub overDexterity: u8,
	pub goodsType: u8,
	pub refCategory: u8,
	pub spEffectCategory: u8,
	#[deku(skip, cond = "version >= 11210015")]
	pub pad3: [u8;1],
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x40: u8,
	pub goodsUseAnim: u8,
	pub opmeMenuType: u8,
	pub useLimitCategory: u8,
	pub replaceCategory: u8,
	pub reserve4: [u8;2],
	#[deku(bits = 1)]
	pub isConsume: u8,
	#[deku(bits = 1)]
	pub isEquip: u8,
	#[deku(bits = 1)]
	pub disable_offline: u8,
	#[deku(bits = 1)]
	pub enable_multi: u8,
	#[deku(bits = 1)]
	pub enable_black: u8,
	#[deku(bits = 1)]
	pub enable_white: u8,
	#[deku(bits = 1)]
	pub enable_gray: u8,
	#[deku(bits = 1)]
	pub enable_live: u8,
	#[deku(bits = 1)]
	pub isSuppleItem: u8,
	#[deku(bits = 1)]
	pub isRemoveItem_forGameClear: u8,
	#[deku(bits = 1)]
	pub isDisableHand: u8,
	#[deku(bits = 1)]
	pub isDeposit: u8,
	#[deku(bits = 1)]
	pub isDiscard: u8,
	#[deku(bits = 1)]
	pub isOnlyOne: u8,
	#[deku(bits = 1)]
	pub isEstablishment: u8,
	#[deku(bits = 1)]
	pub isAutoEquip: u8,
	#[deku(bits = 1)]
	pub isApplySpecialEffect: u8,
	#[deku(bits = 1)]
	pub isEnableFastUseItem: u8,
	#[deku(bits = 1)]
	pub disableUseAtOutOfColiseum: u8,
	#[deku(bits = 1)]
	pub disableUseAtColiseum: u8,
	#[deku(bits = 1)]
	pub disableMultiDropShare: u8,
	#[deku(bits = 1)]
	pub isFixItem: u8,
	#[deku(bits = 1)]
	pub isEnhance: u8,
	#[deku(bits = 1)]
	pub isFullSuppleItem: u8,
	pub syncNumVaryId: u8,
	pub refId_1: i32,
	pub refVirtualWepId: i32,
	pub vagrantItemLotId: i32,
	pub vagrantBonusEneDropItemLotId: i32,
	pub vagrantItemEneDropItemLotId: i32,
	pub castSfxId: i32,
	pub fireSfxId: i32,
	pub effectSfxId: i32,
	#[deku(bits = 1)]
	pub isUseMultiPenaltyOnly: u8,
	#[deku(bits = 1)]
	pub isWarpProhibited: u8,
	#[deku(bits = 1)]
	pub isShieldEnchant: u8,
	#[deku(bits = 1)]
	pub canMultiUse: u8,
	#[deku(bits = 1)]
	pub isUseMultiPlayPreparation: u8,
	#[deku(bits = 1)]
	pub enable_Ladder: u8,
	#[deku(bits = 1)]
	pub isBonfireWarpItem: u8,
	#[deku(bits = 1)]
	pub enable_ActiveBigRune: u8,
	pub suppleType: u8,
	pub autoReplenishType: u8,
	#[deku(bits = 1)]
	pub disableRiding: u8,
	#[deku(bits = 1)]
	pub enableRiding: u8,
	#[deku(bits = 1)]
	pub isSleepCollectionItem: u8,
	#[deku(bits = 2)]
	pub showDialogCondType: u8,
	#[deku(bits = 1)]
	pub isSummonHorse: u8,
	#[deku(bits = 1)]
	pub showLogCondType: u8,
	#[deku(bits = 1)]
	pub isDrop: u8,
	pub maxRepositoryNum: i16,
	pub sortGroupId: u8,
	#[deku(skip, cond = "version >= 11210015", bits = 7)]
	pub pad1_old: u8,
	#[deku(bits = 1)]
	pub isUseNoAttackRegion: u8,
	#[deku(skip, cond = "version < 11210015", bits = 5)]
	pub pad1: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub unknown_0x73_2: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub unknown_0x73_1: u8,
	pub saleValue: i32,
	pub rarity: u8,
	pub useLimitSummonBuddy: u8,
	pub useLimitSpEffectType: i16,
	pub aiUseJudgeId: i32,
	pub consumeMP: i16,
	pub consumeHP: i16,
	pub reinforceGoodsId: i32,
	pub reinforceMaterialId: i32,
	pub reinforcePrice: i32,
	pub useLevel_vowType0: i8,
	pub useLevel_vowType1: i8,
	pub useLevel_vowType2: i8,
	pub useLevel_vowType3: i8,
	pub useLevel_vowType4: i8,
	pub useLevel_vowType5: i8,
	pub useLevel_vowType6: i8,
	pub useLevel_vowType7: i8,
	pub useLevel_vowType8: i8,
	pub useLevel_vowType9: i8,
	pub useLevel_vowType10: i8,
	pub useLevel_vowType11: i8,
	pub useLevel_vowType12: i8,
	pub useLevel_vowType13: i8,
	pub useLevel_vowType14: i8,
	pub useLevel_vowType15: i8,
	pub useLevel: i16,
	pub reserve5: [u8;2],
	pub itemGetTutorialFlagId: i32,
	pub reserve3: [u8;8],
}
