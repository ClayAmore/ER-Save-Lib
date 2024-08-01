use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct GRASS_MAP_SETTINGS_PARAM_ST {
	pub grassType0: i32,
	pub grassType1: i32,
	pub grassType2: i32,
}
