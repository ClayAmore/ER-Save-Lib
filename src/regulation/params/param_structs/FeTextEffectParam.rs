use super::{
	defs::FE_TEXT_EFFECT_PARAM_ST::FE_TEXT_EFFECT_PARAM_ST,
	param_trait::Param
};
pub struct FeTextEffectParam;
impl Param for FeTextEffectParam {
	type ParamType = FE_TEXT_EFFECT_PARAM_ST;
	const PARAM_NAME: &'static str = "FeTextEffectParam";
}
