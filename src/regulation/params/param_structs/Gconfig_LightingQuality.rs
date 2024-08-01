use super::{
	defs::CS_LIGHTING_QUALITY_DETAIL::CS_LIGHTING_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_LightingQuality;
impl Param for Gconfig_LightingQuality {
	type ParamType = CS_LIGHTING_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_LightingQuality";
}
