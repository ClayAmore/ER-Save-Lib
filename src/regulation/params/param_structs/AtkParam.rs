use super::{
	defs::ATK_PARAM_ST::ATK_PARAM_ST,
	param_trait::Param
};
pub struct AtkParam;
impl Param for AtkParam {
	type ParamType = ATK_PARAM_ST;
	const PARAM_NAME: &'static str = "AtkParam";
}
