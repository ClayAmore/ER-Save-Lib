use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct GAME_AREA_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub bonusSoul_single: i32,
	pub bonusSoul_multi: i32,
	pub humanityPointCountFlagIdTop: i32,
	pub humanityDropPoint1: i16,
	pub humanityDropPoint2: i16,
	pub humanityDropPoint3: i16,
	pub humanityDropPoint4: i16,
	pub humanityDropPoint5: i16,
	pub humanityDropPoint6: i16,
	pub humanityDropPoint7: i16,
	pub humanityDropPoint8: i16,
	pub humanityDropPoint9: i16,
	pub humanityDropPoint10: i16,
	pub soloBreakInPoint_Min: i32,
	pub soloBreakInPoint_Max: i32,
	pub defeatBossFlagId_forSignAimList: i32,
	pub displayAimFlagId: i32,
	pub foundBossFlagId: i32,
	pub foundBossTextId: i32,
	pub notFindBossTextId: i32,
	pub bossChallengeFlagId: i32,
	pub defeatBossFlagId: i32,
	pub bossPosX: f32,
	pub bossPosY: f32,
	pub bossPosZ: f32,
	pub bossMapAreaNo: u8,
	pub bossMapBlockNo: u8,
	pub bossMapMapNo: u8,
	#[deku(count = "9")]
	pub reserve: Vec<u8>,
}
