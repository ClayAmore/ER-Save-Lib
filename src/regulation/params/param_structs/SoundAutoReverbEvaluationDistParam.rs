use super::{
	defs::SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST::SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST,
	param_trait::Param
};
pub struct SoundAutoReverbEvaluationDistParam;
impl Param for SoundAutoReverbEvaluationDistParam {
	type ParamType = SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST;
	const PARAM_NAME: &'static str = "SoundAutoReverbEvaluationDistParam";
}
