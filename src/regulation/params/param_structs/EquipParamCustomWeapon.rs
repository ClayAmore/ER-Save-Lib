use super::{
	defs::EQUIP_PARAM_CUSTOM_WEAPON_ST::EQUIP_PARAM_CUSTOM_WEAPON_ST,
	param_trait::Param
};
pub struct EquipParamCustomWeapon;
impl Param for EquipParamCustomWeapon {
	type ParamType = EQUIP_PARAM_CUSTOM_WEAPON_ST;
	const PARAM_NAME: &'static str = "EquipParamCustomWeapon";
}
