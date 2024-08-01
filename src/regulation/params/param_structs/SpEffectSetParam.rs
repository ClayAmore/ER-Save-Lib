use super::{
	defs::SP_EFFECT_SET_PARAM_ST::SP_EFFECT_SET_PARAM_ST,
	param_trait::Param
};
pub struct SpEffectSetParam;
impl Param for SpEffectSetParam {
	type ParamType = SP_EFFECT_SET_PARAM_ST;
	const PARAM_NAME: &'static str = "SpEffectSetParam";
}
