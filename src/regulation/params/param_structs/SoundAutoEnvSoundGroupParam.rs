use super::{
	defs::SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST::SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST,
	param_trait::Param
};
pub struct SoundAutoEnvSoundGroupParam;
impl Param for SoundAutoEnvSoundGroupParam {
	type ParamType = SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST;
	const PARAM_NAME: &'static str = "SoundAutoEnvSoundGroupParam";
}
