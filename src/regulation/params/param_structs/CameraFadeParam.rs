use super::{
	defs::CAMERA_FADE_PARAM_ST::CAMERA_FADE_PARAM_ST,
	param_trait::Param
};
pub struct CameraFadeParam;
impl Param for CameraFadeParam {
	type ParamType = CAMERA_FADE_PARAM_ST;
	const PARAM_NAME: &'static str = "CameraFadeParam";
}
