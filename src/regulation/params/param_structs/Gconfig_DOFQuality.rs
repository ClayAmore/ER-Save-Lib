use super::{
	defs::CS_DOF_QUALITY_DETAIL::CS_DOF_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_DOFQuality;
impl Param for Gconfig_DOFQuality {
	type ParamType = CS_DOF_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_DOFQuality";
}
