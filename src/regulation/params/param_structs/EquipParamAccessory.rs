use super::{
	defs::EQUIP_PARAM_ACCESSORY_ST::EQUIP_PARAM_ACCESSORY_ST,
	param_trait::Param
};
pub struct EquipParamAccessory;
impl Param for EquipParamAccessory {
	type ParamType = EQUIP_PARAM_ACCESSORY_ST;
	const PARAM_NAME: &'static str = "EquipParamAccessory";
}
