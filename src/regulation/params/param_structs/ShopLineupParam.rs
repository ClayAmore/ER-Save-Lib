use super::{
	defs::SHOP_LINEUP_PARAM::SHOP_LINEUP_PARAM,
	param_trait::Param
};
pub struct ShopLineupParam;
impl Param for ShopLineupParam {
	type ParamType = SHOP_LINEUP_PARAM;
	const PARAM_NAME: &'static str = "ShopLineupParam";
}
