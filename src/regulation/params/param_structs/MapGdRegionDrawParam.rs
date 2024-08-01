use super::{
	defs::MAP_GD_REGION_DRAW_PARAM::MAP_GD_REGION_DRAW_PARAM,
	param_trait::Param
};
pub struct MapGdRegionDrawParam;
impl Param for MapGdRegionDrawParam {
	type ParamType = MAP_GD_REGION_DRAW_PARAM;
	const PARAM_NAME: &'static str = "MapGdRegionDrawParam";
}
