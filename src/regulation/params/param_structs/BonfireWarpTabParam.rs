use super::{
	defs::BONFIRE_WARP_TAB_PARAM_ST::BONFIRE_WARP_TAB_PARAM_ST,
	param_trait::Param
};
pub struct BonfireWarpTabParam;
impl Param for BonfireWarpTabParam {
	type ParamType = BONFIRE_WARP_TAB_PARAM_ST;
	const PARAM_NAME: &'static str = "BonfireWarpTabParam";
}
