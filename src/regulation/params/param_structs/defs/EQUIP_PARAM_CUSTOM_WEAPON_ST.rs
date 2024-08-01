use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct EQUIP_PARAM_CUSTOM_WEAPON_ST {
	pub baseWepId: i32,
	pub gemId: i32,
	pub reinforceLv: u8,
	pub pad: [u8;7],
}
