use super::{
	defs::TALK_PARAM_ST::TALK_PARAM_ST,
	param_trait::Param
};
pub struct TalkParam;
impl Param for TalkParam {
	type ParamType = TALK_PARAM_ST;
	const PARAM_NAME: &'static str = "TalkParam";
}
