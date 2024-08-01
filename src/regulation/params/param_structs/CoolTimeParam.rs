use super::{
	defs::COOL_TIME_PARAM_ST::COOL_TIME_PARAM_ST,
	param_trait::Param
};
pub struct CoolTimeParam;
impl Param for CoolTimeParam {
	type ParamType = COOL_TIME_PARAM_ST;
	const PARAM_NAME: &'static str = "CoolTimeParam";
}
