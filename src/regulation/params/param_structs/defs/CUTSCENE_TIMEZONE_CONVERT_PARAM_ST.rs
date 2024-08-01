use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CUTSCENE_TIMEZONE_CONVERT_PARAM_ST {
	pub SrcTimezoneStart: f32,
	pub DstCutscenTime: f32,
}
