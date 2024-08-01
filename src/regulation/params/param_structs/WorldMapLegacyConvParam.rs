use super::{
	defs::WORLD_MAP_LEGACY_CONV_PARAM_ST::WORLD_MAP_LEGACY_CONV_PARAM_ST,
	param_trait::Param
};
pub struct WorldMapLegacyConvParam;
impl Param for WorldMapLegacyConvParam {
	type ParamType = WORLD_MAP_LEGACY_CONV_PARAM_ST;
	const PARAM_NAME: &'static str = "WorldMapLegacyConvParam";
}
