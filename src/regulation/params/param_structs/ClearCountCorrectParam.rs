use super::{
	defs::CLEAR_COUNT_CORRECT_PARAM_ST::CLEAR_COUNT_CORRECT_PARAM_ST,
	param_trait::Param
};
pub struct ClearCountCorrectParam;
impl Param for ClearCountCorrectParam {
	type ParamType = CLEAR_COUNT_CORRECT_PARAM_ST;
	const PARAM_NAME: &'static str = "ClearCountCorrectParam";
}
