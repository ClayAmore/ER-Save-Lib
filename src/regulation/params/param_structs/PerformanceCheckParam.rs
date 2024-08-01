use super::{
	defs::PERFORMANCE_CHECK_PARAM::PERFORMANCE_CHECK_PARAM,
	param_trait::Param
};
pub struct PerformanceCheckParam;
impl Param for PerformanceCheckParam {
	type ParamType = PERFORMANCE_CHECK_PARAM;
	const PARAM_NAME: &'static str = "PerformanceCheckParam";
}
