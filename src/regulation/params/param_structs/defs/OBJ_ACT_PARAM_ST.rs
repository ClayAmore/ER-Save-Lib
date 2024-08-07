use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct OBJ_ACT_PARAM_ST {
	pub actionEnableMsgId: i32,
	pub actionFailedMsgId: i32,
	pub spQualifiedPassEventFlag: i32,
	pub playerAnimId: i32,
	pub chrAnimId: i32,
	pub validDist: i16,
	pub spQualifiedId_old: i16,
	pub spQualifiedId2_old: i16,
	pub objDummyId: u8,
	pub isEventKickSync: u8,
	pub objAnimId: i32,
	pub validPlayerAngle: u8,
	pub spQualifiedType: u8,
	pub spQualifiedType2: u8,
	pub validObjAngle: u8,
	pub chrSorbType: u8,
	pub eventKickTiming: u8,
	pub pad1: [u8;2],
	pub actionButtonParamId: i32,
	pub enableTreasureDelaySec: f32,
	pub preActionSfxDmypolyId: i32,
	pub preActionSfxId: i32,
	#[deku(skip, cond = "version >= 11210015", count = "40")]
	pub pad2_old: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub spQualifiedId_new: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub spQualifiedId2_new: i32,
	#[deku(skip, cond = "version < 11210015", count = "32")]
	pub pad2: Vec<u8>,
}
