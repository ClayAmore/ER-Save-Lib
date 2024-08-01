use super::{
	defs::EQUIP_PARAM_GOODS_ST::EQUIP_PARAM_GOODS_ST,
	param_trait::Param
};
pub struct EquipParamGoods;
impl Param for EquipParamGoods {
	type ParamType = EQUIP_PARAM_GOODS_ST;
	const PARAM_NAME: &'static str = "EquipParamGoods";
}
