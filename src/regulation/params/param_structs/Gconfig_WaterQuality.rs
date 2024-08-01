use super::{
	defs::CS_WATER_QUALITY_DETAIL::CS_WATER_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_WaterQuality;
impl Param for Gconfig_WaterQuality {
	type ParamType = CS_WATER_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_WaterQuality";
}
