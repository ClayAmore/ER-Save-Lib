use super::{
	defs::GAME_AREA_PARAM_ST::GAME_AREA_PARAM_ST,
	param_trait::Param
};
pub struct GameAreaParam;
impl Param for GameAreaParam {
	type ParamType = GAME_AREA_PARAM_ST;
	const PARAM_NAME: &'static str = "GameAreaParam";
}
