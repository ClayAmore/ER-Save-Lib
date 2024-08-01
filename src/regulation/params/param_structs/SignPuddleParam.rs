use super::{
	defs::SIGN_PUDDLE_PARAM_ST::SIGN_PUDDLE_PARAM_ST,
	param_trait::Param
};
pub struct SignPuddleParam;
impl Param for SignPuddleParam {
	type ParamType = SIGN_PUDDLE_PARAM_ST;
	const PARAM_NAME: &'static str = "SignPuddleParam";
}
