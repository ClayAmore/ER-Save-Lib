use super::{
	defs::KEY_ASSIGN_PARAM_ST::KEY_ASSIGN_PARAM_ST,
	param_trait::Param
};
pub struct KeyAssignParam;
impl Param for KeyAssignParam {
	type ParamType = KEY_ASSIGN_PARAM_ST;
	const PARAM_NAME: &'static str = "KeyAssignParam";
}
