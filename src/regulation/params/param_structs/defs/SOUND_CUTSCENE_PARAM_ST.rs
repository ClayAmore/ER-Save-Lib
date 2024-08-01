use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SOUND_CUTSCENE_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub ReverbType: u8,
	pub pad0: [u8;3],
	pub BgmBehaviorTypeOnStart: i16,
	pub OneShotBgmBehaviorOnStart: i16,
	pub PostPlaySeId: i32,
	pub PostPlaySeIdForSkip: i32,
	pub EnterMapMuteStopTimeOnDrawCutscene: f32,
	#[deku(skip, cond = "version >= 11210015 || version < 10501000")]
	pub reserved_old: [u8;8],
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x18: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x19: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x1a: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0x1b: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub reserved: [u8;4],
	#[deku(skip, cond = "version < 10501000")]
	pub reserved2: [u8;4],
}
