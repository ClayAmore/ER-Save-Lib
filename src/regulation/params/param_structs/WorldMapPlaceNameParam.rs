use super::{
	defs::WORLD_MAP_PLACE_NAME_PARAM_ST::WORLD_MAP_PLACE_NAME_PARAM_ST,
	param_trait::Param
};
pub struct WorldMapPlaceNameParam;
impl Param for WorldMapPlaceNameParam {
	type ParamType = WORLD_MAP_PLACE_NAME_PARAM_ST;
	const PARAM_NAME: &'static str = "WorldMapPlaceNameParam";
}
