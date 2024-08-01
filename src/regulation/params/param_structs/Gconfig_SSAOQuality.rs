use super::{
	defs::CS_SSAO_QUALITY_DETAIL::CS_SSAO_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_SSAOQuality;
impl Param for Gconfig_SSAOQuality {
	type ParamType = CS_SSAO_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_SSAOQuality";
}
