use super::{
	defs::FACE_RANGE_PARAM_ST::FACE_RANGE_PARAM_ST,
	param_trait::Param
};
pub struct FaceRangeParam;
impl Param for FaceRangeParam {
	type ParamType = FACE_RANGE_PARAM_ST;
	const PARAM_NAME: &'static str = "FaceRangeParam";
}
