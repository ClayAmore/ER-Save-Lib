use super::{
	defs::BULLET_CREATE_LIMIT_PARAM_ST::BULLET_CREATE_LIMIT_PARAM_ST,
	param_trait::Param
};
pub struct BulletCreateLimitParam;
impl Param for BulletCreateLimitParam {
	type ParamType = BULLET_CREATE_LIMIT_PARAM_ST;
	const PARAM_NAME: &'static str = "BulletCreateLimitParam";
}
