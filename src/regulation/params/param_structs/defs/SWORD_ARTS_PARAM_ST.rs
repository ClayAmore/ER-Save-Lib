use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SWORD_ARTS_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub swordArtsType: u8,
	pub artsSpeedType: u8,
	pub refStatus: i8,
	#[deku(bits = 1)]
	pub isRefRightArts: u8,
	#[deku(bits = 1)]
	pub isGrayoutLeftHand: u8,
	#[deku(bits = 1)]
	pub isGrayoutRightHand: u8,
	#[deku(bits = 1)]
	pub isGrayoutBothHand: u8,
	#[deku(bits = 4)]
	pub reserve2: u8,
	pub usePoint_L1: i8,
	pub usePoint_L2: i8,
	pub usePoint_R1: i8,
	pub usePoint_R2: i8,
	pub textId: i32,
	pub useMagicPoint_L1: i16,
	pub useMagicPoint_L2: i16,
	pub useMagicPoint_R1: i16,
	pub useMagicPoint_R2: i16,
	#[deku(skip, cond = "version >= 10701000")]
	pub shieldIconType: i8,
	#[deku(skip, cond = "version < 10701000")]
	pub swordArtsTypeNew: i16,
	pub iconId: i16,
	pub aiUsageId: i32,
}
