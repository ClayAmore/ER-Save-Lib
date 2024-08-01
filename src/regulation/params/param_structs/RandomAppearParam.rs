use super::{
	defs::RANDOM_APPEAR_PARAM_ST::RANDOM_APPEAR_PARAM_ST,
	param_trait::Param
};
pub struct RandomAppearParam;
impl Param for RandomAppearParam {
	type ParamType = RANDOM_APPEAR_PARAM_ST;
	const PARAM_NAME: &'static str = "RandomAppearParam";
}
