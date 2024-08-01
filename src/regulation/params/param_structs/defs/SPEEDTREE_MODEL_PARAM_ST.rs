use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct SPEEDTREE_MODEL_PARAM_ST {
	pub MinFadeLeaf: f32,
	pub MinFadeFrond: f32,
	pub MinFadeBranch: f32,
	pub MinTranslucencyLeaf: f32,
	pub MaxTranslucencyLeaf: f32,
	pub MinTranslucencyFrond: f32,
	pub MaxTranslucencyFrond: f32,
	pub MinTranslucencyBranch: f32,
	pub MaxTranslucencyBranch: f32,
	pub BillboardBackSpecularWeakenParam: f32,
}
