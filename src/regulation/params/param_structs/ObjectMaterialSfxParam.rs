use super::{
	defs::OBJECT_MATERIAL_SFX_PARAM_ST::OBJECT_MATERIAL_SFX_PARAM_ST,
	param_trait::Param
};
pub struct ObjectMaterialSfxParam;
impl Param for ObjectMaterialSfxParam {
	type ParamType = OBJECT_MATERIAL_SFX_PARAM_ST;
	const PARAM_NAME: &'static str = "ObjectMaterialSfxParam";
}
