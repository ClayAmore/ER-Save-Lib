use super::{
	defs::CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST::CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST,
	param_trait::Param
};
pub struct CutsceneWeatherOverrideGparamIdConvertParam;
impl Param for CutsceneWeatherOverrideGparamIdConvertParam {
	type ParamType = CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST;
	const PARAM_NAME: &'static str = "CutsceneWeatherOverrideGparamIdConvertParam";
}
