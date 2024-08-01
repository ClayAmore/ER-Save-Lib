use super::{
	defs::POSTURE_CONTROL_PARAM_PRO_ST::POSTURE_CONTROL_PARAM_PRO_ST,
	param_trait::Param
};
pub struct PostureControlParam_Pro;
impl Param for PostureControlParam_Pro {
	type ParamType = POSTURE_CONTROL_PARAM_PRO_ST;
	const PARAM_NAME: &'static str = "PostureControlParam_Pro";
}
