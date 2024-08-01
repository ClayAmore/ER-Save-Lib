use super::{
	defs::WEATHER_ASSET_CREATE_PARAM_ST::WEATHER_ASSET_CREATE_PARAM_ST,
	param_trait::Param
};
pub struct WeatherAssetCreateParam;
impl Param for WeatherAssetCreateParam {
	type ParamType = WEATHER_ASSET_CREATE_PARAM_ST;
	const PARAM_NAME: &'static str = "WeatherAssetCreateParam";
}
