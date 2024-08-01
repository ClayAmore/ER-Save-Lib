use super::{
	defs::RUNTIME_BONE_CONTROL_PARAM_ST::RUNTIME_BONE_CONTROL_PARAM_ST,
	param_trait::Param
};
pub struct RuntimeBoneControlParam;
impl Param for RuntimeBoneControlParam {
	type ParamType = RUNTIME_BONE_CONTROL_PARAM_ST;
	const PARAM_NAME: &'static str = "RuntimeBoneControlParam";
}
