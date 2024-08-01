use super::{
	defs::MULTI_ESTUS_FLASK_BONUS_PARAM_ST::MULTI_ESTUS_FLASK_BONUS_PARAM_ST,
	param_trait::Param
};
pub struct MultiEstusFlaskBonusParam;
impl Param for MultiEstusFlaskBonusParam {
	type ParamType = MULTI_ESTUS_FLASK_BONUS_PARAM_ST;
	const PARAM_NAME: &'static str = "MultiEstusFlaskBonusParam";
}
