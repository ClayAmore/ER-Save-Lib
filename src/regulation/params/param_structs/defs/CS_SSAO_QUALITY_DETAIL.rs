use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_SSAO_QUALITY_DETAIL {
	pub enabled: u8,
	pub cs_reprojEnabledType: u8,
	pub cs_upScaleEnabledType: u8,
	pub cs_useNormalEnabledType: u8,
	pub dmy: [u8;1],
}
