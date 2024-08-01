use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST {
	pub SoundNo: i32,
	pub ExpandRange: f32,
	pub FollowSpeed: f32,
	pub FollowRate: f32,
}
