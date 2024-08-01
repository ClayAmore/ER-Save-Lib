use super::{
	defs::ITEMLOT_PARAM_ST::ITEMLOT_PARAM_ST,
	param_trait::Param
};
pub struct ItemLotParam;
impl Param for ItemLotParam {
	type ParamType = ITEMLOT_PARAM_ST;
	const PARAM_NAME: &'static str = "ItemLotParam";
}
