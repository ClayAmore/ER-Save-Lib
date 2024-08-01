use super::{
	defs::WORLD_MAP_POINT_PARAM_ST::WORLD_MAP_POINT_PARAM_ST,
	param_trait::Param
};
pub struct WorldMapPointParam;
impl Param for WorldMapPointParam {
	type ParamType = WORLD_MAP_POINT_PARAM_ST;
	const PARAM_NAME: &'static str = "WorldMapPointParam";
}
