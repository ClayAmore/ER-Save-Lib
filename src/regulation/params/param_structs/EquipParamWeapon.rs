use super::{
	defs::EQUIP_PARAM_WEAPON_ST::EQUIP_PARAM_WEAPON_ST,
	param_trait::Param
};
pub struct EquipParamWeapon;
impl Param for EquipParamWeapon {
	type ParamType = EQUIP_PARAM_WEAPON_ST;
	const PARAM_NAME: &'static str = "EquipParamWeapon";
}
