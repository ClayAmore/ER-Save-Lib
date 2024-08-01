use super::{
	defs::CS_REFLECTION_QUALITY_DETAIL::CS_REFLECTION_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_ReflectionQuality;
impl Param for Gconfig_ReflectionQuality {
	type ParamType = CS_REFLECTION_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_ReflectionQuality";
}
