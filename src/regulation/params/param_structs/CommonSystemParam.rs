use super::{
	defs::COMMON_SYSTEM_PARAM_ST::COMMON_SYSTEM_PARAM_ST,
	param_trait::Param
};
pub struct CommonSystemParam;
impl Param for CommonSystemParam {
	type ParamType = COMMON_SYSTEM_PARAM_ST;
	const PARAM_NAME: &'static str = "CommonSystemParam";
}
