use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct PARTS_DRAW_PARAM_ST {
	pub lv01_BorderDist: f32,
	pub lv01_PlayDist: f32,
	pub lv12_BorderDist: f32,
	pub lv12_PlayDist: f32,
	pub lv23_BorderDist: f32,
	pub lv23_PlayDist: f32,
	pub lv34_BorderDist: f32,
	pub lv34_PlayDist: f32,
	pub lv45_BorderDist: f32,
	pub lv45_PlayDist: f32,
	pub tex_lv01_BorderDist: f32,
	pub tex_lv01_PlayDist: f32,
	#[deku(bits = 1)]
	pub enableCrossFade: i32,
	pub drawDist: f32,
	pub drawFadeRange: f32,
	pub shadowDrawDist: f32,
	pub shadowFadeRange: f32,
	pub motionBlur_BorderDist: f32,
	pub isPointLightShadowSrc: i8,
	pub isDirLightShadowSrc: i8,
	pub isShadowDst: i8,
	pub isShadowOnly: i8,
	pub drawByReflectCam: i8,
	pub drawOnlyReflectCam: i8,
	pub IncludeLodMapLv: i8,
	pub isNoFarClipDraw: u8,
	pub lodType: u8,
	pub shadowDrawLodOffset: i8,
	pub isTraceCameraXZ: u8,
	pub isSkydomeDrawPhase: u8,
	pub DistantViewModel_BorderDist: f32,
	pub DistantViewModel_PlayDist: f32,
	pub LimtedActivate_BorderDist_forGrid: f32,
	pub LimtedActivate_PlayDist_forGrid: f32,
	pub zSortOffsetForNoFarClipDraw: f32,
	pub shadowDrawAlphaTestDist: f32,
	pub fowardDrawEnvmapBlendType: u8,
	pub LBDrawDistScaleParamID: u8,
	#[deku(count = "34")]
	pub resereve: Vec<u8>,
}
