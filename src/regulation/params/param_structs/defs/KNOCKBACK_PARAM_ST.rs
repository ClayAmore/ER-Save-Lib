use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct KNOCKBACK_PARAM_ST {
	pub damage_Min_ContTime: f32,
	pub damage_S_ContTime: f32,
	pub damage_M_ContTime: f32,
	pub damage_L_ContTime: f32,
	pub damage_BlowS_ContTime: f32,
	pub damage_BlowM_ContTime: f32,
	pub damage_Strike_ContTime: f32,
	pub damage_Uppercut_ContTime: f32,
	pub damage_Push_ContTime: f32,
	pub damage_Breath_ContTime: f32,
	pub damage_HeadShot_ContTime: f32,
	pub guard_S_ContTime: f32,
	pub guard_L_ContTime: f32,
	pub guard_LL_ContTime: f32,
	pub guardBrake_ContTime: f32,
	pub damage_Min_DecTime: f32,
	pub damage_S_DecTime: f32,
	pub damage_M_DecTime: f32,
	pub damage_L_DecTime: f32,
	pub damage_BlowS_DecTime: f32,
	pub damage_BlowM_DecTime: f32,
	pub damage_Strike_DecTime: f32,
	pub damage_Uppercut_DecTime: f32,
	pub damage_Push_DecTime: f32,
	pub damage_Breath_DecTime: f32,
	pub damage_HeadShot_DecTime: f32,
	pub guard_S_DecTime: f32,
	pub guard_L_DecTime: f32,
	pub guard_LL_DecTime: f32,
	pub guardBrake_DecTime: f32,
	pub pad: [u8;8],
}
