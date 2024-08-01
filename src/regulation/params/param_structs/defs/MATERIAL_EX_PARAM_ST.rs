use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MATERIAL_EX_PARAM_ST {
	#[deku(count = "32")]
	pub paramName: Vec<u8>,
	pub materialId: i32,
	pub materialParamValue0: f32,
	pub materialParamValue1: f32,
	pub materialParamValue2: f32,
	pub materialParamValue3: f32,
	pub materialParamValue4: f32,
	pub pad: [u8;8],
}
