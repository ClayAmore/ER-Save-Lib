use super::{
	defs::POSTURE_CONTROL_PARAM_GENDER_ST::POSTURE_CONTROL_PARAM_GENDER_ST,
	param_trait::Param
};
pub struct PostureControlParam_Gender;
impl Param for PostureControlParam_Gender {
	type ParamType = POSTURE_CONTROL_PARAM_GENDER_ST;
	const PARAM_NAME: &'static str = "PostureControlParam_Gender";
}
