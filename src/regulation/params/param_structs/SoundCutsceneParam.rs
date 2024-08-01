use super::{
	defs::SOUND_CUTSCENE_PARAM_ST::SOUND_CUTSCENE_PARAM_ST,
	param_trait::Param
};
pub struct SoundCutsceneParam;
impl Param for SoundCutsceneParam {
	type ParamType = SOUND_CUTSCENE_PARAM_ST;
	const PARAM_NAME: &'static str = "SoundCutsceneParam";
}
