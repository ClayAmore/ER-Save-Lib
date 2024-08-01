use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WEP_ABSORP_POS_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub hangPosType: u8,
	pub isSkeletonBind: u8,
	pub pad0: [u8;2],
	pub right_0: i16,
	pub left_0: i16,
	pub both_0: i16,
	pub leftHang_0: i16,
	pub rightHang_0: i16,
	pub right_1: i16,
	pub left_1: i16,
	pub both_1: i16,
	pub leftHang_1: i16,
	pub rightHang_1: i16,
	pub right_2: i16,
	pub left_2: i16,
	pub both_2: i16,
	pub leftHang_2: i16,
	pub rightHang_2: i16,
	pub right_3: i16,
	pub left_3: i16,
	pub both_3: i16,
	pub leftHang_3: i16,
	pub rightHang_3: i16,
	pub wepInvisibleType_0: u8,
	pub wepInvisibleType_1: u8,
	pub wepInvisibleType_2: u8,
	pub wepInvisibleType_3: u8,
	pub leftBoth_0: i16,
	pub leftBoth_1: i16,
	pub leftBoth_2: i16,
	pub leftBoth_3: i16,
	pub dispPosType_right_0: u8,
	pub dispPosType_left_0: u8,
	pub dispPosType_rightBoth_0: u8,
	pub dispPosType_leftBoth_0: u8,
	pub dispPosType_rightHang_0: u8,
	pub dispPosType_leftHang_0: u8,
	pub dispPosType_right_1: u8,
	pub dispPosType_left_1: u8,
	pub dispPosType_rightBoth_1: u8,
	pub dispPosType_leftBoth_1: u8,
	pub dispPosType_rightHang_1: u8,
	pub dispPosType_leftHang_1: u8,
	pub dispPosType_right_2: u8,
	pub dispPosType_left_2: u8,
	pub dispPosType_rightBoth_2: u8,
	pub dispPosType_leftBoth_2: u8,
	pub dispPosType_rightHang_2: u8,
	pub dispPosType_leftHang_2: u8,
	pub dispPosType_right_3: u8,
	pub dispPosType_left_3: u8,
	pub dispPosType_rightBoth_3: u8,
	pub dispPosType_leftBoth_3: u8,
	pub dispPosType_rightHang_3: u8,
	pub dispPosType_leftHang_3: u8,
	#[deku(skip, cond = "version >= 11210015", count = "12")]
	pub reserve_old: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x54: i8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x55: i8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x56: i8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x57: i8,
	#[deku(skip, cond = "version < 11210015")]
	pub reserve: [u8;8],
}
