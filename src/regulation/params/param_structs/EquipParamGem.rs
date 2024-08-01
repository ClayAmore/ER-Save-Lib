use super::{
	defs::EQUIP_PARAM_GEM_ST::EQUIP_PARAM_GEM_ST,
	param_trait::Param
};
pub struct EquipParamGem;
impl Param for EquipParamGem {
	type ParamType = EQUIP_PARAM_GEM_ST;
	const PARAM_NAME: &'static str = "EquipParamGem";
}
