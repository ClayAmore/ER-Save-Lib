use super::{
	defs::CUTSCENE_GPARAM_TIME_PARAM_ST::CUTSCENE_GPARAM_TIME_PARAM_ST,
	param_trait::Param
};
pub struct CutsceneGparamTimeParam;
impl Param for CutsceneGparamTimeParam {
	type ParamType = CUTSCENE_GPARAM_TIME_PARAM_ST;
	const PARAM_NAME: &'static str = "CutsceneGparamTimeParam";
}
