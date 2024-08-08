use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct EQUIP_PARAM_GEM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub iconId: i16,
	pub rank: i8,
	pub sortGroupId: u8,
	pub spEffectId0: i32,
	pub spEffectId1: i32,
	pub spEffectId2: i32,
	pub itemGetTutorialFlagId: i32,
	pub swordArtsParamId: i32,
	pub mountValue: i32,
	pub sellValue: i32,
	pub saleValue: i32,
	pub sortId: i32,
	pub compTrophySedId: i16,
	pub trophySeqId: i16,
	#[deku(bits = 1)]
	pub configurableWepAttr07: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr06: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr05: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr04: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr03: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr02: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr01: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr00: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr15: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr14: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr13: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr12: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr11: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr10: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr09: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr08: u8,
	pub rarity: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr23: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr22: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr21: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr20: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr19: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr18: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr17: u8,
	#[deku(bits = 1)]
	pub configurableWepAttr16: u8,
	#[deku(bits = 1)]
	pub pad: u8,
	#[deku(bits = 1)]
	pub showLogCondType: u8,
	#[deku(bits = 2)]
	pub showDialogCondType: u8,
	#[deku(bits = 1)]
	pub disableMultiDropShare: u8,
	#[deku(bits = 1)]
	pub isDeposit: u8,
	#[deku(bits = 1)]
	pub isDrop: u8,
	#[deku(bits = 1)]
	pub isDiscard: u8,
	pub defaultWepAttr: u8,
	#[deku(skip, cond = "version >= 11210015")]
	pub pad2_old: [u8;2],
	#[deku(skip, cond = "version < 11210015")]
	pub isSpecialSwordArt: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub pad2: [u8;1],
	#[deku(bits = 1)]
	pub canMountWep_SwordDoubleEdge: u8,
	#[deku(bits = 1)]
	pub canMountWep_katana: u8,
	#[deku(bits = 1)]
	pub canMountWep_SaberLarge: u8,
	#[deku(bits = 1)]
	pub canMountWep_SaberNormal: u8,
	#[deku(bits = 1)]
	pub canMountWep_SwordGigantic: u8,
	#[deku(bits = 1)]
	pub canMountWep_SwordLarge: u8,
	#[deku(bits = 1)]
	pub canMountWep_SwordNormal: u8,
	#[deku(bits = 1)]
	pub canMountWep_Dagger: u8,
	#[deku(bits = 1)]
	pub canMountWep_SpearNormal: u8,
	#[deku(bits = 1)]
	pub canMountWep_Flail: u8,
	#[deku(bits = 1)]
	pub canMountWep_HammerLarge: u8,
	#[deku(bits = 1)]
	pub canMountWep_HammerNormal: u8,
	#[deku(bits = 1)]
	pub canMountWep_AxeLarge: u8,
	#[deku(bits = 1)]
	pub canMountWep_AxeNormal: u8,
	#[deku(bits = 1)]
	pub canMountWep_RapierHeavy: u8,
	#[deku(bits = 1)]
	pub canMountWep_SwordPierce: u8,
	#[deku(bits = 1)]
	pub canMountWep_AxhammerLarge: u8,
	#[deku(bits = 1)]
	pub canMountWep_Whip: u8,
	#[deku(bits = 1)]
	pub canMountWep_Claw: u8,
	#[deku(bits = 1)]
	pub canMountWep_Knuckle: u8,
	#[deku(bits = 1)]
	pub canMountWep_Sickle: u8,
	#[deku(bits = 1)]
	pub canMountWep_SpearAxe: u8,
	#[deku(bits = 1)]
	pub canMountWep_SpearHeavy: u8,
	#[deku(bits = 1)]
	pub canMountWep_SpearLarge: u8,
	#[deku(bits = 1)]
	pub canMountWep_Talisman: u8,
	#[deku(bits = 1)]
	pub canMountWep_Sorcery: u8,
	#[deku(bits = 1)]
	pub canMountWep_Staff: u8,
	#[deku(bits = 1)]
	pub canMountWep_Ballista: u8,
	#[deku(bits = 1)]
	pub canMountWep_ClossBow: u8,
	#[deku(bits = 1)]
	pub canMountWep_BowLarge: u8,
	#[deku(bits = 1)]
	pub canMountWep_BowNormal: u8,
	#[deku(bits = 1)]
	pub canMountWep_BowSmall: u8,
	#[deku(skip, cond = "version >= 11210015", bits = 4)]
	pub reserved_canMountWep: u8,
	#[deku(bits = 1)]
	pub canMountWep_Torch: u8,
	#[deku(bits = 1)]
	pub canMountWep_ShieldLarge: u8,
	#[deku(bits = 1)]
	pub canMountWep_ShieldNormal: u8,
	#[deku(bits = 1)]
	pub canMountWep_ShieldSmall: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub canMountWep_ThrowingWeapon: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub canMountWep_ThrustingShield: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub canMountWep_PerfumeBottle: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub canMountWep_HandToHand: u8,
	#[deku(skip, cond = "version >= 11210015")]
	pub reserved2_canMountWep_old: [u8;3],
	#[deku(skip, cond = "version < 11210015", bits = 4)]
	pub reserved_canMountWep_0x3d_4: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub canMountWep_BeastClaw: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub canMountWep_GreatKatana: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub canMountWep_LightGreatsword: u8,
	#[deku(skip, cond = "version < 11210015", bits = 1)]
	pub canMountWep_ReverseHandSword: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub reserved2_canMountWep: [u8;2],
	pub spEffectMsgId0: i32,
	pub spEffectMsgId1: i32,
	pub spEffectId_forAtk0: i32,
	pub spEffectId_forAtk1: i32,
	pub spEffectId_forAtk2: i32,
	pub mountWepTextId: i32,
	pub pad6: [u8;8],
}
