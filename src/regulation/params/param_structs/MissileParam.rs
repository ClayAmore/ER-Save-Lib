use super::{
	defs::MISSILE_PARAM_ST::MISSILE_PARAM_ST,
	param_trait::Param
};
pub struct MissileParam;
impl Param for MissileParam {
	type ParamType = MISSILE_PARAM_ST;
	const PARAM_NAME: &'static str = "MissileParam";
}
