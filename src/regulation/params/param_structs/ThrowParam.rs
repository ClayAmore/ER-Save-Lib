use super::{
	defs::THROW_PARAM_ST::THROW_PARAM_ST,
	param_trait::Param
};
pub struct ThrowParam;
impl Param for ThrowParam {
	type ParamType = THROW_PARAM_ST;
	const PARAM_NAME: &'static str = "ThrowParam";
}
