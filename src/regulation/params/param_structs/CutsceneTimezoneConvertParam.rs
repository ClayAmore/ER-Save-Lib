use super::{
	defs::CUTSCENE_TIMEZONE_CONVERT_PARAM_ST::CUTSCENE_TIMEZONE_CONVERT_PARAM_ST,
	param_trait::Param
};
pub struct CutsceneTimezoneConvertParam;
impl Param for CutsceneTimezoneConvertParam {
	type ParamType = CUTSCENE_TIMEZONE_CONVERT_PARAM_ST;
	const PARAM_NAME: &'static str = "CutsceneTimezoneConvertParam";
}
