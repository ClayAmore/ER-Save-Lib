use super::{
	defs::CS_DECAL_QUALITY_DETAIL::CS_DECAL_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_DecalQuality;
impl Param for Gconfig_DecalQuality {
	type ParamType = CS_DECAL_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_DecalQuality";
}
