use super::{
	defs::BULLET_PARAM_ST::BULLET_PARAM_ST,
	param_trait::Param
};
pub struct BulletParam;
impl Param for BulletParam {
	type ParamType = BULLET_PARAM_ST;
	const PARAM_NAME: &'static str = "BulletParam";
}
