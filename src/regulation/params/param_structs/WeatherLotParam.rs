use super::{
	defs::WEATHER_LOT_PARAM_ST::WEATHER_LOT_PARAM_ST,
	param_trait::Param
};
pub struct WeatherLotParam;
impl Param for WeatherLotParam {
	type ParamType = WEATHER_LOT_PARAM_ST;
	const PARAM_NAME: &'static str = "WeatherLotParam";
}
