use super::{
	defs::SOUND_AUTO_REVERB_SELECT_PARAM_ST::SOUND_AUTO_REVERB_SELECT_PARAM_ST,
	param_trait::Param
};
pub struct SoundAutoReverbSelectParam;
impl Param for SoundAutoReverbSelectParam {
	type ParamType = SOUND_AUTO_REVERB_SELECT_PARAM_ST;
	const PARAM_NAME: &'static str = "SoundAutoReverbSelectParam";
}
