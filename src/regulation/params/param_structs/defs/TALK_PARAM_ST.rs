use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct TALK_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub msgId: i32,
	pub voiceId: i32,
	pub spEffectId0: i32,
	pub motionId0: i32,
	pub spEffectId1: i32,
	pub motionId1: i32,
	pub returnPos: i32,
	pub reactionId: i32,
	pub eventId: i32,
	pub msgId_female: i32,
	pub voiceId_female: i32,
	pub lipSyncStart: i16,
	pub lipSyncTime: i16,
	pub pad2: [u8;4],
	pub timeout: f32,
	pub talkAnimationId: i32,
	#[deku(bits = 7)]
	pub pad3: u8,
	#[deku(bits = 1)]
	pub isForceDisp: u8,
	#[deku(count = "31")]
	pub pad1: Vec<u8>,
}
