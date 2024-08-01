use super::{
	defs::BEHAVIOR_PARAM_ST::BEHAVIOR_PARAM_ST,
	param_trait::Param
};
pub struct BehaviorParam;
impl Param for BehaviorParam {
	type ParamType = BEHAVIOR_PARAM_ST;
	const PARAM_NAME: &'static str = "BehaviorParam";
}
