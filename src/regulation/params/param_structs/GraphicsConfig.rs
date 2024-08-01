use super::{
	defs::CS_GRAPHICS_CONFIG_PARAM_ST::CS_GRAPHICS_CONFIG_PARAM_ST,
	param_trait::Param
};
pub struct GraphicsConfig;
impl Param for GraphicsConfig {
	type ParamType = CS_GRAPHICS_CONFIG_PARAM_ST;
	const PARAM_NAME: &'static str = "GraphicsConfig";
}
