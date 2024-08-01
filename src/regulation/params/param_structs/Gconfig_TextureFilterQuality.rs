use super::{
	defs::CS_TEXTURE_FILTER_QUALITY_DETAIL::CS_TEXTURE_FILTER_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_TextureFilterQuality;
impl Param for Gconfig_TextureFilterQuality {
	type ParamType = CS_TEXTURE_FILTER_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_TextureFilterQuality";
}
