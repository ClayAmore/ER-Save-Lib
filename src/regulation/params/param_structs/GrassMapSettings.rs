use super::{
	defs::GRASS_MAP_SETTINGS_PARAM_ST::GRASS_MAP_SETTINGS_PARAM_ST,
	param_trait::Param
};
pub struct GrassMapSettings;
impl Param for GrassMapSettings {
	type ParamType = GRASS_MAP_SETTINGS_PARAM_ST;
	const PARAM_NAME: &'static str = "GrassMapSettings";
}
