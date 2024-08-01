use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct BUDGET_PARAM_ST {
	pub vram_all: f32,
	pub vram_mapobj_tex: f32,
	pub vram_mapobj_mdl: f32,
	pub vram_map: f32,
	pub vram_chr: f32,
	pub vram_parts: f32,
	pub vram_sfx: f32,
	pub vram_chr_tex: f32,
	pub vram_chr_mdl: f32,
	pub vram_parts_tex: f32,
	pub vram_parts_mdl: f32,
	pub vram_sfx_tex: f32,
	pub vram_sfx_mdl: f32,
	pub vram_gi: f32,
	pub vram_menu_tex: f32,
	pub vram_decal_rt: f32,
	pub vram_decal: f32,
	pub reserve_0: [u8;4],
	pub vram_other_tex: f32,
	pub vram_other_mdl: f32,
	pub havok_anim: f32,
	pub havok_ins: f32,
	pub havok_hit: f32,
	pub vram_other: f32,
	pub vram_detail_all: f32,
	pub vram_chr_and_parts: f32,
	pub havok_navimesh: f32,
	#[deku(count = "24")]
	pub reserve_1: Vec<u8>,
}
