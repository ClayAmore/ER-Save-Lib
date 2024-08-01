use super::{
	defs::PLAYER_COMMON_PARAM_ST::PLAYER_COMMON_PARAM_ST,
	param_trait::Param
};
pub struct PlayerCommonParam;
impl Param for PlayerCommonParam {
	type ParamType = PLAYER_COMMON_PARAM_ST;
	const PARAM_NAME: &'static str = "PlayerCommonParam";
}
