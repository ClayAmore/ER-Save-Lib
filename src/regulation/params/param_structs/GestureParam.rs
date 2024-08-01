use super::{
	defs::GESTURE_PARAM_ST::GESTURE_PARAM_ST,
	param_trait::Param
};
pub struct GestureParam;
impl Param for GestureParam {
	type ParamType = GESTURE_PARAM_ST;
	const PARAM_NAME: &'static str = "GestureParam";
}
