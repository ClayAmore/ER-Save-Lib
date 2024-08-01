use super::{
	defs::LOAD_BALANCER_PARAM_ST::LOAD_BALANCER_PARAM_ST,
	param_trait::Param
};
pub struct LoadBalancerParam;
impl Param for LoadBalancerParam {
	type ParamType = LOAD_BALANCER_PARAM_ST;
	const PARAM_NAME: &'static str = "LoadBalancerParam";
}
