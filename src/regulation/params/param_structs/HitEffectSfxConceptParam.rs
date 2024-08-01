use super::{
	defs::HIT_EFFECT_SFX_CONCEPT_PARAM_ST::HIT_EFFECT_SFX_CONCEPT_PARAM_ST,
	param_trait::Param
};
pub struct HitEffectSfxConceptParam;
impl Param for HitEffectSfxConceptParam {
	type ParamType = HIT_EFFECT_SFX_CONCEPT_PARAM_ST;
	const PARAM_NAME: &'static str = "HitEffectSfxConceptParam";
}
