use super::{
	defs::CHR_ACTIVATE_CONDITION_PARAM_ST::CHR_ACTIVATE_CONDITION_PARAM_ST,
	param_trait::Param
};
pub struct ChrActivateConditionParam;
impl Param for ChrActivateConditionParam {
	type ParamType = CHR_ACTIVATE_CONDITION_PARAM_ST;
	const PARAM_NAME: &'static str = "ChrActivateConditionParam";
}
