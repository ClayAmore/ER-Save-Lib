use super::{
	defs::REINFORCE_PARAM_PROTECTOR_ST::REINFORCE_PARAM_PROTECTOR_ST,
	param_trait::Param
};
pub struct ReinforceParamProtector;
impl Param for ReinforceParamProtector {
	type ParamType = REINFORCE_PARAM_PROTECTOR_ST;
	const PARAM_NAME: &'static str = "ReinforceParamProtector";
}
