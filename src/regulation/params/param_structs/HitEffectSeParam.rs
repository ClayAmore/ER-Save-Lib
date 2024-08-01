use super::{
	defs::HIT_EFFECT_SE_PARAM_ST::HIT_EFFECT_SE_PARAM_ST,
	param_trait::Param
};
pub struct HitEffectSeParam;
impl Param for HitEffectSeParam {
	type ParamType = HIT_EFFECT_SE_PARAM_ST;
	const PARAM_NAME: &'static str = "HitEffectSeParam";
}
