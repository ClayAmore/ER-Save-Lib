use super::{
	defs::EQUIP_MTRL_SET_PARAM_ST::EQUIP_MTRL_SET_PARAM_ST,
	param_trait::Param
};
pub struct EquipMtrlSetParam;
impl Param for EquipMtrlSetParam {
	type ParamType = EQUIP_MTRL_SET_PARAM_ST;
	const PARAM_NAME: &'static str = "EquipMtrlSetParam";
}
