use super::{
	defs::WHITE_SIGN_COOL_TIME_PARAM_ST::WHITE_SIGN_COOL_TIME_PARAM_ST,
	param_trait::Param
};
pub struct WhiteSignCoolTimeParam;
impl Param for WhiteSignCoolTimeParam {
	type ParamType = WHITE_SIGN_COOL_TIME_PARAM_ST;
	const PARAM_NAME: &'static str = "WhiteSignCoolTimeParam";
}
