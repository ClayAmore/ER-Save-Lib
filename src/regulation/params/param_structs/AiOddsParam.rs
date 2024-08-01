use super::{
	defs::AI_ODDS_PARAM::AI_ODDS_PARAM,
	param_trait::Param
};
pub struct AiOddsParam;
impl Param for AiOddsParam {
	type ParamType = AI_ODDS_PARAM;
	const PARAM_NAME: &'static str = "AiOddsParam";
}
