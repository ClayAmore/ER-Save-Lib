use super::{
	defs::RANDOM_APPEAR_EDIT_PARAM_ST::RANDOM_APPEAR_EDIT_PARAM_ST,
	param_trait::Param
};
pub struct RandomAppearEditParam;
impl Param for RandomAppearEditParam {
	type ParamType = RANDOM_APPEAR_EDIT_PARAM_ST;
	const PARAM_NAME: &'static str = "RandomAppearEditParam";
}
