use super::{
	defs::AI_ATTACK_PARAM_ST::AI_ATTACK_PARAM_ST,
	param_trait::Param
};
pub struct AIAttackParam;
impl Param for AIAttackParam {
	type ParamType = AI_ATTACK_PARAM_ST;
	const PARAM_NAME: &'static str = "AIAttackParam";
}
