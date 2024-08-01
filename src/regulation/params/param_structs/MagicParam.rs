use super::{
	defs::MAGIC_PARAM_ST::MAGIC_PARAM_ST,
	param_trait::Param
};
pub struct MagicParam;
impl Param for MagicParam {
	type ParamType = MAGIC_PARAM_ST;
	const PARAM_NAME: &'static str = "MagicParam";
}
