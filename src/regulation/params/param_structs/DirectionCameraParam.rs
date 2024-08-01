use super::{
	defs::DIRECTION_CAMERA_PARAM_ST::DIRECTION_CAMERA_PARAM_ST,
	param_trait::Param
};
pub struct DirectionCameraParam;
impl Param for DirectionCameraParam {
	type ParamType = DIRECTION_CAMERA_PARAM_ST;
	const PARAM_NAME: &'static str = "DirectionCameraParam";
}
