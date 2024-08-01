use super::{
	defs::BONFIRE_WARP_SUB_CATEGORY_PARAM_ST::BONFIRE_WARP_SUB_CATEGORY_PARAM_ST,
	param_trait::Param
};
pub struct BonfireWarpSubCategoryParam;
impl Param for BonfireWarpSubCategoryParam {
	type ParamType = BONFIRE_WARP_SUB_CATEGORY_PARAM_ST;
	const PARAM_NAME: &'static str = "BonfireWarpSubCategoryParam";
}
