use super::{
	defs::AI_STANDARD_INFO_BANK::AI_STANDARD_INFO_BANK,
	param_trait::Param
};
pub struct AiStandardInfo;
impl Param for AiStandardInfo {
	type ParamType = AI_STANDARD_INFO_BANK;
	const PARAM_NAME: &'static str = "AiStandardInfo";
}
