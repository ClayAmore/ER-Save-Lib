use super::{
	defs::GAME_SYSTEM_COMMON_PARAM_ST::GAME_SYSTEM_COMMON_PARAM_ST,
	param_trait::Param
};
pub struct GameSystemCommonParam;
impl Param for GameSystemCommonParam {
	type ParamType = GAME_SYSTEM_COMMON_PARAM_ST;
	const PARAM_NAME: &'static str = "GameSystemCommonParam";
}
