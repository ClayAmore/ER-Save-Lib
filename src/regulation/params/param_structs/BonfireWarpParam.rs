use super::{
	defs::BONFIRE_WARP_PARAM_ST::BONFIRE_WARP_PARAM_ST,
	param_trait::Param
};
pub struct BonfireWarpParam;
impl Param for BonfireWarpParam {
	type ParamType = BONFIRE_WARP_PARAM_ST;
	const PARAM_NAME: &'static str = "BonfireWarpParam";
}
