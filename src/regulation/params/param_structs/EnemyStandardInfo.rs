use super::{
	defs::ENEMY_STANDARD_INFO_BANK::ENEMY_STANDARD_INFO_BANK,
	param_trait::Param
};
pub struct EnemyStandardInfo;
impl Param for EnemyStandardInfo {
	type ParamType = ENEMY_STANDARD_INFO_BANK;
	const PARAM_NAME: &'static str = "EnemyStandardInfo";
}
