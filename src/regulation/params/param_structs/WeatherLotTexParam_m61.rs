use super::{
	defs::WEATHER_LOT_TEX_PARAM_ST_DLC02::WEATHER_LOT_TEX_PARAM_ST_DLC02,
	param_trait::Param
};
pub struct WeatherLotTexParam_m61;
impl Param for WeatherLotTexParam_m61 {
	type ParamType = WEATHER_LOT_TEX_PARAM_ST_DLC02;
	const PARAM_NAME: &'static str = "WeatherLotTexParam_m61";
}
