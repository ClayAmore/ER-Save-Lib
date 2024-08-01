use super::{
	defs::MULTI_SOUL_BONUS_RATE_PARAM_ST::MULTI_SOUL_BONUS_RATE_PARAM_ST,
	param_trait::Param
};
pub struct MultiSoulBonusRateParam;
impl Param for MultiSoulBonusRateParam {
	type ParamType = MULTI_SOUL_BONUS_RATE_PARAM_ST;
	const PARAM_NAME: &'static str = "MultiSoulBonusRateParam";
}
