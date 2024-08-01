use super::{
	defs::EQUIP_PARAM_PROTECTOR_ST::EQUIP_PARAM_PROTECTOR_ST,
	param_trait::Param
};
pub struct EquipParamProtector;
impl Param for EquipParamProtector {
	type ParamType = EQUIP_PARAM_PROTECTOR_ST;
	const PARAM_NAME: &'static str = "EquipParamProtector";
}
