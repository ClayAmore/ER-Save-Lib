use super::{
	defs::GPARAM_GRID_REGION_INFO_PARAM_ST::GPARAM_GRID_REGION_INFO_PARAM_ST,
	param_trait::Param
};
pub struct GparamGridRegionInfo;
impl Param for GparamGridRegionInfo {
	type ParamType = GPARAM_GRID_REGION_INFO_PARAM_ST;
	const PARAM_NAME: &'static str = "GparamGridRegionInfo";
}
