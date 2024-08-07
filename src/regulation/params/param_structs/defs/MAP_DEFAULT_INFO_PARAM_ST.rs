use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MAP_DEFAULT_INFO_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub EnableFastTravelEventFlagId: i32,
	pub WeatherLotTimeOffsetIngameSeconds: i32,
	pub WeatherCreateAssetLimitId: i8,
	pub MapAiSightType: u8,
	pub SoundIndoorType: u8,
	pub ReverbDefaultType: i8,
	pub BgmPlaceInfo: i16,
	pub EnvPlaceInfo: i16,
	pub MapAdditionalSoundBankId: i32,
	pub MapHeightForSound: i16,
	pub IsEnableBlendTimezoneEnvmap: u8,
	pub OverrideGIResolution_XSS: i8,
	pub MapLoHiChangeBorderDist_XZ: f32,
	pub MapLoHiChangeBorderDist_Y: f32,
	pub MapLoHiChangePlayDist: f32,
	pub MapAutoDrawGroupBackFacePixelNum: i32,
	pub PlayerLigntScale: f32,
	pub IsEnableTimezonnePlayerLigntScale: u8,
	pub isDisableAutoCliffWind: u8,
	pub OpenChrActivateThreshold: i16,
	pub MapMimicryEstablishmentParamId: i32,
	pub OverrideGIResolution_XSX: i8,
	pub Reserve: [u8;7],
}
