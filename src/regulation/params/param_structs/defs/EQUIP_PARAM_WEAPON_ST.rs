use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct EQUIP_PARAM_WEAPON_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub behaviorVariationId: i32,
	pub sortId: i32,
	pub wanderingEquipId: i32,
	pub weight: f32,
	pub weaponWeightRate: f32,
	pub fixPrice: i32,
	pub reinforcePrice: i32,
	pub sellValue: i32,
	pub correctStrength: f32,
	pub correctAgility: f32,
	pub correctMagic: f32,
	pub correctFaith: f32,
	pub physGuardCutRate: f32,
	pub magGuardCutRate: f32,
	pub fireGuardCutRate: f32,
	pub thunGuardCutRate: f32,
	pub spEffectBehaviorId0: i32,
	pub spEffectBehaviorId1: i32,
	pub spEffectBehaviorId2: i32,
	pub residentSpEffectId: i32,
	pub residentSpEffectId1: i32,
	pub residentSpEffectId2: i32,
	pub materialSetId: i32,
	pub originEquipWep: i32,
	pub originEquipWep1: i32,
	pub originEquipWep2: i32,
	pub originEquipWep3: i32,
	pub originEquipWep4: i32,
	pub originEquipWep5: i32,
	pub originEquipWep6: i32,
	pub originEquipWep7: i32,
	pub originEquipWep8: i32,
	pub originEquipWep9: i32,
	pub originEquipWep10: i32,
	pub originEquipWep11: i32,
	pub originEquipWep12: i32,
	pub originEquipWep13: i32,
	pub originEquipWep14: i32,
	pub originEquipWep15: i32,
	pub weakA_DamageRate: f32,
	pub weakB_DamageRate: f32,
	pub weakC_DamageRate: f32,
	pub weakD_DamageRate: f32,
	pub sleepGuardResist_MaxCorrect: f32,
	pub madnessGuardResist_MaxCorrect: f32,
	pub saWeaponDamage: f32,
	pub equipModelId: i16,
	pub iconId: i16,
	pub durability: i16,
	pub durabilityMax: i16,
	pub attackThrowEscape: i16,
	pub parryDamageLife: i16,
	pub attackBasePhysics: i16,
	pub attackBaseMagic: i16,
	pub attackBaseFire: i16,
	pub attackBaseThunder: i16,
	pub attackBaseStamina: i16,
	pub guardAngle: i16,
	pub saDurability: f32,
	pub staminaGuardDef: i16,
	pub reinforceTypeId: i16,
	pub trophySGradeId: i16,
	pub trophySeqId: i16,
	pub throwAtkRate: i16,
	pub bowDistRate: i16,
	pub equipModelCategory: u8,
	pub equipModelGender: u8,
	pub weaponCategory: u8,
	pub wepmotionCategory: u8,
	pub guardmotionCategory: u8,
	pub atkMaterial: u8,
	pub defSeMaterial1: i16,
	pub correctType_Physics: u8,
	pub spAttribute: u8,
	pub spAtkcategory: i16,
	pub wepmotionOneHandId: u8,
	pub wepmotionBothHandId: u8,
	pub properStrength: u8,
	pub properAgility: u8,
	pub properMagic: u8,
	pub properFaith: u8,
	pub overStrength: u8,
	pub attackBaseParry: u8,
	pub defenseBaseParry: u8,
	pub guardBaseRepel: u8,
	pub attackBaseRepel: u8,
	pub guardCutCancelRate: i8,
	pub guardLevel: i8,
	pub slashGuardCutRate: i8,
	pub blowGuardCutRate: i8,
	pub thrustGuardCutRate: i8,
	pub poisonGuardResist: i8,
	pub diseaseGuardResist: i8,
	pub bloodGuardResist: i8,
	pub curseGuardResist: i8,
	pub atkAttribute: u8,
	#[deku(bits = 1)]
	pub enableMagic: u8,
	#[deku(bits = 1)]
	pub enableParry: u8,
	#[deku(bits = 1)]
	pub enableGuard: u8,
	#[deku(bits = 1)]
	pub boltSlotEquipable: u8,
	#[deku(bits = 1)]
	pub arrowSlotEquipable: u8,
	#[deku(bits = 1)]
	pub bothHandEquipable: u8,
	#[deku(bits = 1)]
	pub leftHandEquipable: u8,
	#[deku(bits = 1)]
	pub rightHandEquipable: u8,
	#[deku(bits = 1)]
	pub isEnhance: u8,
	#[deku(bits = 1)]
	pub isThrustAttackType: u8,
	#[deku(bits = 1)]
	pub isSlashAttackType: u8,
	#[deku(bits = 1)]
	pub isBlowAttackType: u8,
	#[deku(bits = 1)]
	pub isNormalAttackType: u8,
	#[deku(bits = 1)]
	pub enableVowMagic: u8,
	#[deku(bits = 1)]
	pub enableMiracle: u8,
	#[deku(bits = 1)]
	pub enableSorcery: u8,
	#[deku(bits = 1)]
	pub isVersusGhostWep: u8,
	#[deku(bits = 1)]
	pub lanternWep: u8,
	#[deku(bits = 1)]
	pub simpleModelForDlc: u8,
	#[deku(bits = 1)]
	pub isDarkHand: u8,
	#[deku(bits = 1)]
	pub disableRepair: u8,
	#[deku(bits = 1)]
	pub disableBaseChangeReset: u8,
	#[deku(bits = 1)]
	pub isCustom: u8,
	#[deku(bits = 1)]
	pub isHeroPointCorrect: u8,
	#[deku(bits = 1)]
	pub isDeposit: u8,
	#[deku(bits = 1)]
	pub isDragonSlayer: u8,
	#[deku(bits = 6)]
	pub baseChangeCategory: u8,
	#[deku(bits = 1)]
	pub disableGemAttr: u8,
	#[deku(bits = 2)]
	pub showDialogCondType: u8,
	#[deku(bits = 1)]
	pub enableThrow: u8,
	#[deku(bits = 1)]
	pub showLogCondType: u8,
	#[deku(bits = 1)]
	pub isDrop: u8,
	#[deku(bits = 1)]
	pub isDiscard: u8,
	#[deku(bits = 1)]
	pub disableMultiDropShare: u8,
	pub defSfxMaterial1: i16,
	pub wepCollidableType0: u8,
	pub wepCollidableType1: u8,
	pub postureControlId_Right: u8,
	pub postureControlId_Left: u8,
	pub traceSfxId0: i32,
	pub traceDmyIdHead0: i32,
	pub traceDmyIdTail0: i32,
	pub traceSfxId1: i32,
	pub traceDmyIdHead1: i32,
	pub traceDmyIdTail1: i32,
	pub traceSfxId2: i32,
	pub traceDmyIdHead2: i32,
	pub traceDmyIdTail2: i32,
	pub traceSfxId3: i32,
	pub traceDmyIdHead3: i32,
	pub traceDmyIdTail3: i32,
	pub traceSfxId4: i32,
	pub traceDmyIdHead4: i32,
	pub traceDmyIdTail4: i32,
	pub traceSfxId5: i32,
	pub traceDmyIdHead5: i32,
	pub traceDmyIdTail5: i32,
	pub traceSfxId6: i32,
	pub traceDmyIdHead6: i32,
	pub traceDmyIdTail6: i32,
	pub traceSfxId7: i32,
	pub traceDmyIdHead7: i32,
	pub traceDmyIdTail7: i32,
	pub defSfxMaterial2: i16,
	pub defSeMaterial2: i16,
	pub absorpParamId: i32,
	pub toughnessCorrectRate: f32,
	#[deku(skip, cond = "version >= 11210015", bits = 3)]
	pub unk1: u8,
	#[deku(bits = 1)]
	pub invisibleOnRemo: u8,
	#[deku(bits = 1)]
	pub isEnableEmergencyStep: u8,
	#[deku(bits = 1)]
	pub isAutoEquip: u8,
	#[deku(bits = 1)]
	pub isDualBlade: u8,
	#[deku(bits = 1)]
	pub isValidTough_ProtSADmg: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub unknown_0x17c_7: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub unknown_0x17c_6: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub unknown_0x17c_5: u8,
	pub correctType_Magic: u8,
	pub correctType_Fire: u8,
	pub correctType_Thunder: u8,
	pub weakE_DamageRate: f32,
	pub weakF_DamageRate: f32,
	pub darkGuardCutRate: f32,
	pub attackBaseDark: i16,
	pub correctType_Dark: u8,
	pub correctType_Poison: u8,
	pub sortGroupId: u8,
	pub atkAttribute2: u8,
	pub sleepGuardResist: i8,
	pub madnessGuardResist: i8,
	pub correctType_Blood: u8,
	pub properLuck: u8,
	pub freezeGuardResist: i8,
	pub autoReplenishType: u8,
	pub swordArtsParamId: i32,
	pub correctLuck: f32,
	pub arrowBoltEquipId: i32,
	pub DerivationLevelType: u8,
	pub enchantSfxSize: u8,
	pub wepType: i16,
	pub physGuardCutRate_MaxCorrect: f32,
	pub magGuardCutRate_MaxCorrect: f32,
	pub fireGuardCutRate_MaxCorrect: f32,
	pub thunGuardCutRate_MaxCorrect: f32,
	pub darkGuardCutRate_MaxCorrect: f32,
	pub poisonGuardResist_MaxCorrect: f32,
	pub diseaseGuardResist_MaxCorrect: f32,
	pub bloodGuardResist_MaxCorrect: f32,
	pub curseGuardResist_MaxCorrect: f32,
	pub freezeGuardResist_MaxCorrect: f32,
	pub staminaGuardDef_MaxCorrect: f32,
	pub residentSfxId_1: i32,
	pub residentSfxId_2: i32,
	pub residentSfxId_3: i32,
	pub residentSfxId_4: i32,
	pub residentSfx_DmyId_1: i32,
	pub residentSfx_DmyId_2: i32,
	pub residentSfx_DmyId_3: i32,
	pub residentSfx_DmyId_4: i32,
	pub staminaConsumptionRate: f32,
	pub vsPlayerDmgCorrectRate_Physics: f32,
	pub vsPlayerDmgCorrectRate_Magic: f32,
	pub vsPlayerDmgCorrectRate_Fire: f32,
	pub vsPlayerDmgCorrectRate_Thunder: f32,
	pub vsPlayerDmgCorrectRate_Dark: f32,
	pub vsPlayerDmgCorrectRate_Poison: f32,
	pub vsPlayerDmgCorrectRate_Blood: f32,
	pub vsPlayerDmgCorrectRate_Freeze: f32,
	pub attainmentWepStatusStr: i32,
	pub attainmentWepStatusDex: i32,
	pub attainmentWepStatusMag: i32,
	pub attainmentWepStatusFai: i32,
	pub attainmentWepStatusLuc: i32,
	pub attackElementCorrectId: i32,
	pub saleValue: i32,
	pub reinforceShopCategory: u8,
	pub maxArrowQuantity: u8,
	#[deku(bits = 1)]
	pub isSoulParamIdChange_model3: u8,
	#[deku(bits = 1)]
	pub isSoulParamIdChange_model2: u8,
	#[deku(bits = 1)]
	pub isSoulParamIdChange_model1: u8,
	#[deku(bits = 1)]
	pub isSoulParamIdChange_model0: u8,
	#[deku(bits = 1)]
	pub residentSfx_4_IsVisibleForHang: u8,
	#[deku(bits = 1)]
	pub residentSfx_3_IsVisibleForHang: u8,
	#[deku(bits = 1)]
	pub residentSfx_2_IsVisibleForHang: u8,
	#[deku(bits = 1)]
	pub residentSfx_1_IsVisibleForHang: u8,
	pub wepSeIdOffset: i8,
	pub baseChangePrice: i32,
	pub levelSyncCorrectId: i16,
	pub correctType_Sleep: u8,
	pub correctType_Madness: u8,
	pub rarity: u8,
	pub gemMountType: u8,
	pub wepRegainHp: i16,
	pub spEffectMsgId0: i32,
	pub spEffectMsgId1: i32,
	pub spEffectMsgId2: i32,
	pub originEquipWep16: i32,
	pub originEquipWep17: i32,
	pub originEquipWep18: i32,
	pub originEquipWep19: i32,
	pub originEquipWep20: i32,
	pub originEquipWep21: i32,
	pub originEquipWep22: i32,
	pub originEquipWep23: i32,
	pub originEquipWep24: i32,
	pub originEquipWep25: i32,
	pub vsPlayerDmgCorrectRate_Sleep: f32,
	pub vsPlayerDmgCorrectRate_Madness: f32,
	pub saGuardCutRate: f32,
	pub defMaterialVariationValue: u8,
	pub spAttributeVariationValue: u8,
	pub stealthAtkRate: i16,
	pub vsPlayerDmgCorrectRate_Disease: f32,
	pub vsPlayerDmgCorrectRate_Curse: f32,
	#[deku(skip, cond = "version >= 11210015")]
	pub pad_old: [u8;8],
	#[deku(skip, cond = "version < 11210015")]
	pub restrictSpecialSwordArt: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub pad: [u8;7],
}
