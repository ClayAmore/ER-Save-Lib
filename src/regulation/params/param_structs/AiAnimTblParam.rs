use super::{
	defs::AI_ANIM_TBL_PARAM::AI_ANIM_TBL_PARAM,
	param_trait::Param
};
pub struct AiAnimTblParam;
impl Param for AiAnimTblParam {
	type ParamType = AI_ANIM_TBL_PARAM;
	const PARAM_NAME: &'static str = "AiAnimTblParam";
}
