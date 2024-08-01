use super::{
	defs::WORLD_MAP_PIECE_PARAM_ST::WORLD_MAP_PIECE_PARAM_ST,
	param_trait::Param
};
pub struct WorldMapPieceParam;
impl Param for WorldMapPieceParam {
	type ParamType = WORLD_MAP_PIECE_PARAM_ST;
	const PARAM_NAME: &'static str = "WorldMapPieceParam";
}
