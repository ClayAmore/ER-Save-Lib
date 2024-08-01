use super::{
	defs::ATTACK_ELEMENT_CORRECT_PARAM_ST::ATTACK_ELEMENT_CORRECT_PARAM_ST,
	param_trait::Param
};
pub struct AttackElementCorrectParam;
impl Param for AttackElementCorrectParam {
	type ParamType = ATTACK_ELEMENT_CORRECT_PARAM_ST;
	const PARAM_NAME: &'static str = "AttackElementCorrectParam";
}
