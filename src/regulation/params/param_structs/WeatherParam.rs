use super::{
	defs::WEATHER_PARAM_ST::WEATHER_PARAM_ST,
	param_trait::Param
};
pub struct WeatherParam;
impl Param for WeatherParam {
	type ParamType = WEATHER_PARAM_ST;
	const PARAM_NAME: &'static str = "WeatherParam";
}
