use super::{
	defs::SP_EFFECT_PARAM_ST::SP_EFFECT_PARAM_ST,
	param_trait::Param
};
pub struct SpEffect;
impl Param for SpEffect {
	type ParamType = SP_EFFECT_PARAM_ST;
	const PARAM_NAME: &'static str = "SpEffect";
}
