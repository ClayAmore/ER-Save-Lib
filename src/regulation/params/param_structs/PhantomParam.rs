use super::{
	defs::PHANTOM_PARAM_ST::PHANTOM_PARAM_ST,
	param_trait::Param
};
pub struct PhantomParam;
impl Param for PhantomParam {
	type ParamType = PHANTOM_PARAM_ST;
	const PARAM_NAME: &'static str = "PhantomParam";
}
