use super::{
	defs::SE_MATERIAL_CONVERT_PARAM_ST::SE_MATERIAL_CONVERT_PARAM_ST,
	param_trait::Param
};
pub struct SeMaterialConvertParam;
impl Param for SeMaterialConvertParam {
	type ParamType = SE_MATERIAL_CONVERT_PARAM_ST;
	const PARAM_NAME: &'static str = "SeMaterialConvertParam";
}
