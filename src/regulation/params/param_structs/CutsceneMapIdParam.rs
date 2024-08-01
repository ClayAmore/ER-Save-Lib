use super::{
	defs::CUTSCENE_MAP_ID_PARAM_ST::CUTSCENE_MAP_ID_PARAM_ST,
	param_trait::Param
};
pub struct CutsceneMapIdParam;
impl Param for CutsceneMapIdParam {
	type ParamType = CUTSCENE_MAP_ID_PARAM_ST;
	const PARAM_NAME: &'static str = "CutsceneMapIdParam";
}
