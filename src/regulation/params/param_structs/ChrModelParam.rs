use super::{
	defs::CHR_MODEL_PARAM_ST::CHR_MODEL_PARAM_ST,
	param_trait::Param
};
pub struct ChrModelParam;
impl Param for ChrModelParam {
	type ParamType = CHR_MODEL_PARAM_ST;
	const PARAM_NAME: &'static str = "ChrModelParam";
}
