use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SHOP_LINEUP_PARAM {
	pub equipId: i32,
	pub value: i32,
	pub mtrlId: i32,
	pub eventFlag_forStock: i32,
	pub eventFlag_forRelease: i32,
	pub sellQuantity: i16,
	pub pad3: [u8;1],
	pub equipType: u8,
	pub costType: u8,
	pub pad1: [u8;1],
	pub setNum: i16,
	pub value_Add: i32,
	pub value_Magnification: f32,
	pub iconId: i32,
	pub nameMsgId: i32,
	pub menuTitleMsgId: i32,
	pub menuIconId: i16,
	pub pad2: [u8;2],
}
