use super::{
	defs::FINAL_DAMAGE_RATE_PARAM_ST::FINAL_DAMAGE_RATE_PARAM_ST,
	param_trait::Param
};
pub struct FinalDamageRateParam;
impl Param for FinalDamageRateParam {
	type ParamType = FINAL_DAMAGE_RATE_PARAM_ST;
	const PARAM_NAME: &'static str = "FinalDamageRateParam";
}
