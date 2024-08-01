use super::{
	defs::WEATHER_ASSET_REPLACE_PARAM_ST::WEATHER_ASSET_REPLACE_PARAM_ST,
	param_trait::Param
};
pub struct WeatherAssetReplaceParam;
impl Param for WeatherAssetReplaceParam {
	type ParamType = WEATHER_ASSET_REPLACE_PARAM_ST;
	const PARAM_NAME: &'static str = "WeatherAssetReplaceParam";
}
