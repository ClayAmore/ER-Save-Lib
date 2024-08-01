use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct EQUIP_PARAM_ACCESSORY_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub refId: i32,
	pub sfxVariationId: i32,
	pub weight: f32,
	pub behaviorId: i32,
	pub basicPrice: i32,
	pub sellValue: i32,
	pub sortId: i32,
	pub qwcId: i32,
	pub equipModelId: i16,
	pub iconId: i16,
	pub shopLv: i16,
	pub trophySGradeId: i16,
	pub trophySeqId: i16,
	pub equipModelCategory: u8,
	pub equipModelGender: u8,
	pub accessoryCategory: u8,
	pub refCategory: u8,
	pub spEffectCategory: u8,
	pub sortGroupId: u8,
	pub vagrantItemLotId: i32,
	pub vagrantBonusEneDropItemLotId: i32,
	pub vagrantItemEneDropItemLotId: i32,
	#[deku(bits = 1)]
	pub isDeposit: u8,
	#[deku(bits = 1)]
	pub isEquipOutBrake: u8,
	#[deku(bits = 1)]
	pub disableMultiDropShare: u8,
	#[deku(bits = 1)]
	pub isDiscard: u8,
	#[deku(bits = 1)]
	pub isDrop: u8,
	#[deku(bits = 1)]
	pub showLogCondType: u8,
	#[deku(bits = 2)]
	pub showDialogCondType: u8,
	pub rarity: u8,
	pub pad2: [u8;2],
	pub saleValue: i32,
	pub accessoryGroup: i16,
	pub pad3: [u8;1],
	pub compTrophySedId: i8,
	pub residentSpEffectId1: i32,
	pub residentSpEffectId2: i32,
	pub residentSpEffectId3: i32,
	pub residentSpEffectId4: i32,
	pub pad1: [u8;4],
}
