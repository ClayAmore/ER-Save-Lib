use super::{
	defs::CS_RAYTRACING_QUALITY_DETAIL::CS_RAYTRACING_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_RaytracingQuality;
impl Param for Gconfig_RaytracingQuality {
	type ParamType = CS_RAYTRACING_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_RaytracingQuality";
}
