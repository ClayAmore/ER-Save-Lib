use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct EQUIP_MTRL_SET_PARAM_ST {
	pub materialId01: i32,
	pub materialId02: i32,
	pub materialId03: i32,
	pub materialId04: i32,
	pub materialId05: i32,
	pub materialId06: i32,
	pub pad_id: [u8;8],
	pub itemNum01: i8,
	pub itemNum02: i8,
	pub itemNum03: i8,
	pub itemNum04: i8,
	pub itemNum05: i8,
	pub itemNum06: i8,
	pub pad_num: [u8;2],
	pub materialCate01: u8,
	pub materialCate02: u8,
	pub materialCate03: u8,
	pub materialCate04: u8,
	pub materialCate05: u8,
	pub materialCate06: u8,
	pub pad_cate: [u8;2],
	#[deku(bits = 1)]
	pub isDisableDispNum01: u8,
	#[deku(bits = 1)]
	pub isDisableDispNum02: u8,
	#[deku(bits = 1)]
	pub isDisableDispNum03: u8,
	#[deku(bits = 1)]
	pub isDisableDispNum04: u8,
	#[deku(bits = 1)]
	pub isDisableDispNum05: u8,
	#[deku(bits = 1)]
	pub isDisableDispNum06: u8,
	pub pad: [u8;3],
}
