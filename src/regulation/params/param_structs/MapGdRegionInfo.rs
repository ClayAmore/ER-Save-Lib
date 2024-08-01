use super::{
	defs::MAP_GD_REGION_ID_PARAM_ST::MAP_GD_REGION_ID_PARAM_ST,
	param_trait::Param
};
pub struct MapGdRegionInfo;
impl Param for MapGdRegionInfo {
	type ParamType = MAP_GD_REGION_ID_PARAM_ST;
	const PARAM_NAME: &'static str = "MapGdRegionInfo";
}
