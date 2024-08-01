use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct ACTIONBUTTON_PARAM_ST {
	pub regionType: u8,
	pub category: u8,
	pub padding1: [u8;2],
	pub dummyPoly1: i32,
	pub dummyPoly2: i32,
	pub radius: f32,
	pub angle: i32,
	pub depth: f32,
	pub width: f32,
	pub height: f32,
	pub baseHeightOffset: f32,
	pub angleCheckType: u8,
	pub padding2: [u8;3],
	pub allowAngle: i32,
	pub spotDummyPoly: i32,
	pub textBoxType: u8,
	pub padding3: [u8;2],
	#[deku(bits = 1)]
	pub padding5: u8,
	#[deku(bits = 1)]
	pub isInvalidForRide: u8,
	#[deku(bits = 1)]
	pub isGrayoutForRide: u8,
	#[deku(bits = 1)]
	pub isInvalidForCrouching: u8,
	#[deku(bits = 1)]
	pub isGrayoutForCrouching: u8,
	#[deku(bits = 3)]
	pub padding4: u8,
	pub textId: i32,
	pub invalidFlag: i32,
	pub grayoutFlag: i32,
	pub overrideActionButtonIdForRide: i32,
	pub execInvalidTime: f32,
	#[deku(count = "28")]
	pub padding6: Vec<u8>,
}
