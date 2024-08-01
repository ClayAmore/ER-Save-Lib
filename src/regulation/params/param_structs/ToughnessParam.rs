use super::{
	defs::TOUGHNESS_PARAM_ST::TOUGHNESS_PARAM_ST,
	param_trait::Param
};
pub struct ToughnessParam;
impl Param for ToughnessParam {
	type ParamType = TOUGHNESS_PARAM_ST;
	const PARAM_NAME: &'static str = "ToughnessParam";
}
