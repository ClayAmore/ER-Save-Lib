use super::{
	defs::SE_ACTIVATION_RANGE_PARAM_ST::SE_ACTIVATION_RANGE_PARAM_ST,
	param_trait::Param
};
pub struct SeActivationRangeParam;
impl Param for SeActivationRangeParam {
	type ParamType = SE_ACTIVATION_RANGE_PARAM_ST;
	const PARAM_NAME: &'static str = "SeActivationRangeParam";
}
