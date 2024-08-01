use super::{
	defs::LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST::LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST,
	param_trait::Param
};
pub struct LoadBalancerDrawDistScaleParam;
impl Param for LoadBalancerDrawDistScaleParam {
	type ParamType = LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST;
	const PARAM_NAME: &'static str = "LoadBalancerDrawDistScaleParam";
}
