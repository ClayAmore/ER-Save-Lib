use super::{
	defs::LOCK_CAM_PARAM_ST::LOCK_CAM_PARAM_ST,
	param_trait::Param
};
pub struct LockCamParam;
impl Param for LockCamParam {
	type ParamType = LOCK_CAM_PARAM_ST;
	const PARAM_NAME: &'static str = "LockCamParam";
}
