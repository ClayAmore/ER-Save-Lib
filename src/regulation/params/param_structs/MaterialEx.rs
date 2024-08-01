use super::{
	defs::MATERIAL_EX_PARAM_ST::MATERIAL_EX_PARAM_ST,
	param_trait::Param
};
pub struct MaterialEx;
impl Param for MaterialEx {
	type ParamType = MATERIAL_EX_PARAM_ST;
	const PARAM_NAME: &'static str = "MaterialEx";
}
