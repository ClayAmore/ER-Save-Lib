use super::{
	defs::CS_SHADER_QUALITY_DETAIL::CS_SHADER_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_ShaderQuality;
impl Param for Gconfig_ShaderQuality {
	type ParamType = CS_SHADER_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_ShaderQuality";
}
