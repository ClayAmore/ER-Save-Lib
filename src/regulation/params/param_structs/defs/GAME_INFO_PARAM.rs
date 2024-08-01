use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct GAME_INFO_PARAM {
	pub titleMsgId: i32,
	pub contentMsgId: i32,
	pub value: i32,
	pub sortId: i32,
	pub eventId: i32,
	#[deku(count = "12")]
	pub Pad: Vec<u8>,
}
