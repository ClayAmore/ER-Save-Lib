use super::{
	defs::WAYPOINT_PARAM_ST::WAYPOINT_PARAM_ST,
	param_trait::Param
};
pub struct WaypointParam;
impl Param for WaypointParam {
	type ParamType = WAYPOINT_PARAM_ST;
	const PARAM_NAME: &'static str = "WaypointParam";
}
