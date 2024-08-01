use super::{
	defs::MAP_GRID_CREATE_HEIGHT_LIMIT_DETAIL_INFO_PARAM_ST::MAP_GRID_CREATE_HEIGHT_LIMIT_DETAIL_INFO_PARAM_ST,
	param_trait::Param
};
pub struct MapGridCreateHeightDetailLimitInfo;
impl Param for MapGridCreateHeightDetailLimitInfo {
	type ParamType = MAP_GRID_CREATE_HEIGHT_LIMIT_DETAIL_INFO_PARAM_ST;
	const PARAM_NAME: &'static str = "MapGridCreateHeightDetailLimitInfo";
}
