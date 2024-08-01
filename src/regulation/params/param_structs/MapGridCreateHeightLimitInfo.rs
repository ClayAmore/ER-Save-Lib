use super::{
	defs::MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST::MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST,
	param_trait::Param
};
pub struct MapGridCreateHeightLimitInfo;
impl Param for MapGridCreateHeightLimitInfo {
	type ParamType = MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST;
	const PARAM_NAME: &'static str = "MapGridCreateHeightLimitInfo";
}
