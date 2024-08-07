use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct CUTSCENE_GPARAM_WEATHER_PARAM_ST {
	#[deku(bits = 6)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_Debug: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub DstWeather_Sunny: i16,
	pub DstWeather_ClearSky: i16,
	pub DstWeather_WeakCloudy: i16,
	pub DstWeather_Cloud: i16,
	pub DstWeather_Rain: i16,
	pub DstWeather_HeavyRain: i16,
	pub DstWeather_Storm: i16,
	pub DstWeather_StormForBattle: i16,
	pub DstWeather_Snow: i16,
	pub DstWeather_HeavySnow: i16,
	pub DstWeather_Fog: i16,
	pub DstWeather_HeavyFog: i16,
	pub DstWeather_SandStorm: i16,
	pub DstWeather_HeavyFogRain: i16,
	pub PostPlayIngameWeather: i16,
	pub IndoorOutdoorType: u8,
	pub TakeOverDstWeather_Sunny: u8,
	pub TakeOverDstWeather_ClearSky: u8,
	pub TakeOverDstWeather_WeakCloudy: u8,
	pub TakeOverDstWeather_Cloud: u8,
	pub TakeOverDstWeather_Rain: u8,
	pub TakeOverDstWeather_HeavyRain: u8,
	pub TakeOverDstWeather_Storm: u8,
	pub TakeOverDstWeather_StormForBattle: u8,
	pub TakeOverDstWeather_Snow: u8,
	pub TakeOverDstWeather_HeavySnow: u8,
	pub TakeOverDstWeather_Fog: u8,
	pub TakeOverDstWeather_HeavyFog: u8,
	pub TakeOverDstWeather_SandStorm: u8,
	pub TakeOverDstWeather_HeavyFogRain: u8,
	pub reserved: [u8;7],
	pub DstWeather_Snowstorm: i16,
	pub DstWeather_LightningStorm: i16,
	pub DstWeather_Reserved3: i16,
	pub DstWeather_Reserved4: i16,
	pub DstWeather_Reserved5: i16,
	pub DstWeather_Reserved6: i16,
	pub DstWeather_Reserved7: i16,
	pub DstWeather_Reserved8: i16,
	pub TakeOverDstWeather_Snowstorm: u8,
	pub TakeOverDstWeather_LightningStorm: u8,
	pub TakeOverDstWeather_Reserved3: u8,
	pub TakeOverDstWeather_Reserved4: u8,
	pub TakeOverDstWeather_Reserved5: u8,
	pub TakeOverDstWeather_Reserved6: u8,
	pub TakeOverDstWeather_Reserved7: u8,
	pub TakeOverDstWeather_Reserved8: u8,
	pub IsEnableApplyMapGdRegionIdForGparam: u8,
	pub reserved2: [u8;1],
	pub OverrideMapGdRegionId: i16,
	#[deku(count = "12")]
	pub reserved1: Vec<u8>,
}
