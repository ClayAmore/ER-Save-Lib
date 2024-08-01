use super::{
	defs::CUTSCENE_GPARAM_WEATHER_PARAM_ST::CUTSCENE_GPARAM_WEATHER_PARAM_ST,
	param_trait::Param
};
pub struct CutsceneGparamWeatherParam;
impl Param for CutsceneGparamWeatherParam {
	type ParamType = CUTSCENE_GPARAM_WEATHER_PARAM_ST;
	const PARAM_NAME: &'static str = "CutsceneGparamWeatherParam";
}
