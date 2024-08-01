use super::{
	defs::GRASS_LOD_RANGE_PARAM_ST::GRASS_LOD_RANGE_PARAM_ST,
	param_trait::Param
};
pub struct GrassLodRangeParam;
impl Param for GrassLodRangeParam {
	type ParamType = GRASS_LOD_RANGE_PARAM_ST;
	const PARAM_NAME: &'static str = "GrassLodRangeParam";
}
