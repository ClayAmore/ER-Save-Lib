use super::{
	defs::CS_SHADOW_QUALITY_DETAIL::CS_SHADOW_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_ShadowQuality;
impl Param for Gconfig_ShadowQuality {
	type ParamType = CS_SHADOW_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_ShadowQuality";
}
