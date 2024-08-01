use super::{
	defs::GRAPHICS_COMMON_PARAM_ST::GRAPHICS_COMMON_PARAM_ST,
	param_trait::Param
};
pub struct GraphicsCommonParam;
impl Param for GraphicsCommonParam {
	type ParamType = GRAPHICS_COMMON_PARAM_ST;
	const PARAM_NAME: &'static str = "GraphicsCommonParam";
}
