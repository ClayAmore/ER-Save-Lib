use super::{
	defs::MULTI_PLAY_CORRECTION_PARAM_ST::MULTI_PLAY_CORRECTION_PARAM_ST,
	param_trait::Param
};
pub struct MultiPlayCorrectionParam;
impl Param for MultiPlayCorrectionParam {
	type ParamType = MULTI_PLAY_CORRECTION_PARAM_ST;
	const PARAM_NAME: &'static str = "MultiPlayCorrectionParam";
}
