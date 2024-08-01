use super::{
	defs::KNOCKBACK_PARAM_ST::KNOCKBACK_PARAM_ST,
	param_trait::Param
};
pub struct KnockBackParam;
impl Param for KnockBackParam {
	type ParamType = KNOCKBACK_PARAM_ST;
	const PARAM_NAME: &'static str = "KnockBackParam";
}
