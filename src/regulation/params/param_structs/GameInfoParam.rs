use super::{
	defs::GAME_INFO_PARAM::GAME_INFO_PARAM,
	param_trait::Param
};
pub struct GameInfoParam;
impl Param for GameInfoParam {
	type ParamType = GAME_INFO_PARAM;
	const PARAM_NAME: &'static str = "GameInfoParam";
}
