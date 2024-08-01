use super::{
	defs::MAP_NAME_TEX_PARAM_ST::MAP_NAME_TEX_PARAM_ST,
	param_trait::Param
};
pub struct MapNameTexParam;
impl Param for MapNameTexParam {
	type ParamType = MAP_NAME_TEX_PARAM_ST;
	const PARAM_NAME: &'static str = "MapNameTexParam";
}
