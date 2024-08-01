use super::{
	defs::SP_EFFECT_VFX_PARAM_ST::SP_EFFECT_VFX_PARAM_ST,
	param_trait::Param
};
pub struct SpEffectVfx;
impl Param for SpEffectVfx {
	type ParamType = SP_EFFECT_VFX_PARAM_ST;
	const PARAM_NAME: &'static str = "SpEffectVfx";
}
