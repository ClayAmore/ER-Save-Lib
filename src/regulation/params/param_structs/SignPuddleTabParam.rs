use super::{
	defs::SIGN_PUDDLE_TAB_PARAM_ST::SIGN_PUDDLE_TAB_PARAM_ST,
	param_trait::Param
};
pub struct SignPuddleTabParam;
impl Param for SignPuddleTabParam {
	type ParamType = SIGN_PUDDLE_TAB_PARAM_ST;
	const PARAM_NAME: &'static str = "SignPuddleTabParam";
}
