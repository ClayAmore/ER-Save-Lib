use super::{
	defs::LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST::LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST,
	param_trait::Param
};
pub struct LoadBalancerNewDrawDistScaleParam;
impl Param for LoadBalancerNewDrawDistScaleParam {
	type ParamType = LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST;
	const PARAM_NAME: &'static str = "LoadBalancerNewDrawDistScaleParam";
}
