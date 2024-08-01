use super::{
	defs::CHR_EQUIP_MODEL_PARAM_ST::CHR_EQUIP_MODEL_PARAM_ST,
	param_trait::Param
};
pub struct ChrEquipModelParam;
impl Param for ChrEquipModelParam {
	type ParamType = CHR_EQUIP_MODEL_PARAM_ST;
	const PARAM_NAME: &'static str = "ChrEquipModelParam";
}
