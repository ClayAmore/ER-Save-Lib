use super::{
	defs::POSTURE_CONTROL_PARAM_WEP_LEFT_ST::POSTURE_CONTROL_PARAM_WEP_LEFT_ST,
	param_trait::Param
};
pub struct PostureControlParam_WepLeft;
impl Param for PostureControlParam_WepLeft {
	type ParamType = POSTURE_CONTROL_PARAM_WEP_LEFT_ST;
	const PARAM_NAME: &'static str = "PostureControlParam_WepLeft";
}
