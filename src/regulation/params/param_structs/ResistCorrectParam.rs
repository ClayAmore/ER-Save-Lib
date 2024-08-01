use super::{
	defs::RESIST_CORRECT_PARAM_ST::RESIST_CORRECT_PARAM_ST,
	param_trait::Param
};
pub struct ResistCorrectParam;
impl Param for ResistCorrectParam {
	type ParamType = RESIST_CORRECT_PARAM_ST;
	const PARAM_NAME: &'static str = "ResistCorrectParam";
}
