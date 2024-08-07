use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SOUND_CHR_PHYSICS_SE_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub ContactLandSeId: i32,
	pub ContactLandAddSeId: i32,
	pub ContactLandPlayNum: i32,
	pub IsEnablePlayCountPerRigid: u8,
	pub pad: [u8;3],
	pub ContactLandMinImpuse: f32,
	pub ContactLandMinSpeed: f32,
	pub ContactPlayerSeId: i32,
	pub ContactPlayerMinImpuse: f32,
	pub ContactPlayerMinSpeed: f32,
	pub ContactCheckRigidIdx0: i8,
	pub ContactCheckRigidIdx1: i8,
	pub ContactCheckRigidIdx2: i8,
	pub ContactCheckRigidIdx3: i8,
	pub ContactCheckRigidIdx4: i8,
	pub ContactCheckRigidIdx5: i8,
	pub ContactCheckRigidIdx6: i8,
	pub ContactCheckRigidIdx7: i8,
	pub ContactCheckRigidIdx8: i8,
	pub ContactCheckRigidIdx9: i8,
	pub ContactCheckRigidIdx10: i8,
	pub ContactCheckRigidIdx11: i8,
	pub ContactCheckRigidIdx12: i8,
	pub ContactCheckRigidIdx13: i8,
	pub ContactCheckRigidIdx14: i8,
	pub ContactCheckRigidIdx15: i8,
}
