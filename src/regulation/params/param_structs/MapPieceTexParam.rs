use super::{
	defs::MAP_PIECE_TEX_PARAM_ST::MAP_PIECE_TEX_PARAM_ST,
	param_trait::Param
};
pub struct MapPieceTexParam;
impl Param for MapPieceTexParam {
	type ParamType = MAP_PIECE_TEX_PARAM_ST;
	const PARAM_NAME: &'static str = "MapPieceTexParam";
}
