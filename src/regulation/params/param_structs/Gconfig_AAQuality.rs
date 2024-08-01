use super::{
	defs::CS_AA_QUALITY_DETAIL::CS_AA_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_AAQuality;
impl Param for Gconfig_AAQuality {
	type ParamType = CS_AA_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_AAQuality";
}
