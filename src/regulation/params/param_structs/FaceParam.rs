use super::{
	defs::FACE_PARAM_ST::FACE_PARAM_ST,
	param_trait::Param
};
pub struct FaceParam;
impl Param for FaceParam {
	type ParamType = FACE_PARAM_ST;
	const PARAM_NAME: &'static str = "FaceParam";
}
