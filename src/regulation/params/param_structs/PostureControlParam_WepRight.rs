use super::{
	defs::POSTURE_CONTROL_PARAM_WEP_RIGHT_ST::POSTURE_CONTROL_PARAM_WEP_RIGHT_ST,
	param_trait::Param
};
pub struct PostureControlParam_WepRight;
impl Param for PostureControlParam_WepRight {
	type ParamType = POSTURE_CONTROL_PARAM_WEP_RIGHT_ST;
	const PARAM_NAME: &'static str = "PostureControlParam_WepRight";
}
