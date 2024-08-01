use super::{
	defs::GRASS_TYPE_PARAM_ST::GRASS_TYPE_PARAM_ST,
	param_trait::Param
};
pub struct GrassTypeParam;
impl Param for GrassTypeParam {
	type ParamType = GRASS_TYPE_PARAM_ST;
	const PARAM_NAME: &'static str = "GrassTypeParam";
}
