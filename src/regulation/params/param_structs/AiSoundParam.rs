use super::{
	defs::AI_SOUND_PARAM_ST::AI_SOUND_PARAM_ST,
	param_trait::Param
};
pub struct AiSoundParam;
impl Param for AiSoundParam {
	type ParamType = AI_SOUND_PARAM_ST;
	const PARAM_NAME: &'static str = "AiSoundParam";
}
