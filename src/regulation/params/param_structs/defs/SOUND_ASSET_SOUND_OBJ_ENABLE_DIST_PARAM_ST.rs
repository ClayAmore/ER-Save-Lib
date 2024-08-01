use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST {
	pub SoundObjEnableDist: f32,
}
