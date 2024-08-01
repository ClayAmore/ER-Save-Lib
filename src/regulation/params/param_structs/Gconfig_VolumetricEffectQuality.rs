use super::{
	defs::CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL::CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_VolumetricEffectQuality;
impl Param for Gconfig_VolumetricEffectQuality {
	type ParamType = CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_VolumetricEffectQuality";
}
