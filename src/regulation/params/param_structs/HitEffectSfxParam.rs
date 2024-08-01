use super::{
	defs::HIT_EFFECT_SFX_PARAM_ST::HIT_EFFECT_SFX_PARAM_ST,
	param_trait::Param
};
pub struct HitEffectSfxParam;
impl Param for HitEffectSfxParam {
	type ParamType = HIT_EFFECT_SFX_PARAM_ST;
	const PARAM_NAME: &'static str = "HitEffectSfxParam";
}
