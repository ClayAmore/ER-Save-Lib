use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct THROW_PARAM_ST {
	pub AtkChrId: i32,
	pub DefChrId: i32,
	pub Dist: f32,
	pub DiffAngMin: f32,
	pub DiffAngMax: f32,
	pub upperYRange: f32,
	pub lowerYRange: f32,
	pub diffAngMyToDef: f32,
	pub throwTypeId: i32,
	pub atkAnimId: i32,
	pub defAnimId: i32,
	pub escHp: i16,
	pub selfEscCycleTime: i16,
	pub sphereCastRadiusRateTop: i16,
	pub sphereCastRadiusRateLow: i16,
	pub PadType: u8,
	pub AtkEnableState: u8,
	pub throwFollowingType: u8,
	pub pad2: [u8;1],
	pub throwType: u8,
	pub selfEscCycleCnt: u8,
	pub dmyHasChrDirType: u8,
	#[deku(bits = 1)]
	pub isTurnAtker: u8,
	#[deku(bits = 1)]
	pub isSkipWepCate: u8,
	#[deku(bits = 1)]
	pub isSkipSphereCast: u8,
	#[deku(bits = 1)]
	pub isEnableCorrectPos_forThrowAdjust: u8,
	#[deku(bits = 1)]
	pub isEnableThrowFollowingFallAssist: u8,
	#[deku(bits = 1)]
	pub isEnableThrowFollowingFeedback: u8,
	#[deku(bits = 2)]
	pub pad0: u8,
	pub atkSorbDmyId: i16,
	pub defSorbDmyId: i16,
	pub Dist_start: f32,
	pub DiffAngMin_start: f32,
	pub DiffAngMax_start: f32,
	pub upperYRange_start: f32,
	pub lowerYRange_start: f32,
	pub diffAngMyToDef_start: f32,
	pub judgeRangeBasePosDmyId1: i32,
	pub judgeRangeBasePosDmyId2: i32,
	pub adsrobModelPosInterpolationTime: f32,
	pub throwFollowingEndEasingTime: f32,
	#[deku(count = "24")]
	pub pad1: Vec<u8>,
}
