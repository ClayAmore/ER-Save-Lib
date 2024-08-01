use super::{
	defs::CS_EFFECT_QUALITY_DETAIL::CS_EFFECT_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_EffectQuality;
impl Param for Gconfig_EffectQuality {
	type ParamType = CS_EFFECT_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_EffectQuality";
}
