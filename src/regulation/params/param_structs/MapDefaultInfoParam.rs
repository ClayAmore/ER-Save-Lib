use super::{
	defs::MAP_DEFAULT_INFO_PARAM_ST::MAP_DEFAULT_INFO_PARAM_ST,
	param_trait::Param
};
pub struct MapDefaultInfoParam;
impl Param for MapDefaultInfoParam {
	type ParamType = MAP_DEFAULT_INFO_PARAM_ST;
	const PARAM_NAME: &'static str = "MapDefaultInfoParam";
}
