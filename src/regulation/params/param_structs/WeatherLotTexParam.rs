use super::{
	defs::WEATHER_LOT_TEX_PARAM_ST::WEATHER_LOT_TEX_PARAM_ST,
	param_trait::Param
};
pub struct WeatherLotTexParam;
impl Param for WeatherLotTexParam {
	type ParamType = WEATHER_LOT_TEX_PARAM_ST;
	const PARAM_NAME: &'static str = "WeatherLotTexParam";
}
