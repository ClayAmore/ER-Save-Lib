use super::{
	defs::ENEMY_COMMON_PARAM_ST::ENEMY_COMMON_PARAM_ST,
	param_trait::Param
};
pub struct EnemyCommonParam;
impl Param for EnemyCommonParam {
	type ParamType = ENEMY_COMMON_PARAM_ST;
	const PARAM_NAME: &'static str = "EnemyCommonParam";
}
