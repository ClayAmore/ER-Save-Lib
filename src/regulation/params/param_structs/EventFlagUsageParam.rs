use super::{
	defs::EVENT_FLAG_USAGE_PARAM_ST::EVENT_FLAG_USAGE_PARAM_ST,
	param_trait::Param
};
pub struct EventFlagUsageParam;
impl Param for EventFlagUsageParam {
	type ParamType = EVENT_FLAG_USAGE_PARAM_ST;
	const PARAM_NAME: &'static str = "EventFlagUsageParam";
}
