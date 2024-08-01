use super::{
	defs::CS_MOTION_BLUR_QUALITY_DETAIL::CS_MOTION_BLUR_QUALITY_DETAIL,
	param_trait::Param
};
pub struct Gconfig_MotionBlurQuality;
impl Param for Gconfig_MotionBlurQuality {
	type ParamType = CS_MOTION_BLUR_QUALITY_DETAIL;
	const PARAM_NAME: &'static str = "Gconfig_MotionBlurQuality";
}
