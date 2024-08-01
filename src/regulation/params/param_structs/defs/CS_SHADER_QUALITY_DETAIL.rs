use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_SHADER_QUALITY_DETAIL {
	pub sssEnabled: u8,
	pub tessellationEnabled: u8,
	pub highPrecisionNormalEnabled: u8,
	pub dmy: [u8;1],
}
