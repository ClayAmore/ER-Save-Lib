use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CS_RAYTRACING_QUALITY_DETAIL {
	pub enableRaytraceAO: u8,
	pub enableRaytraceShadows: u8,
	pub Unk0x02: u8,
	pub Unk0x03: u8,
	pub UnkFloat0x04: f32,
	pub Unk0x08: i32,
	pub UnkFloat0x0C: f32,
	pub Unk0x10: i32,
	pub penumbraSize: f32,
	pub renderDistance: f32,
}
