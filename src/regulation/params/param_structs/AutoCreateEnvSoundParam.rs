use super::{
	defs::AUTO_CREATE_ENV_SOUND_PARAM_ST::AUTO_CREATE_ENV_SOUND_PARAM_ST,
	param_trait::Param
};
pub struct AutoCreateEnvSoundParam;
impl Param for AutoCreateEnvSoundParam {
	type ParamType = AUTO_CREATE_ENV_SOUND_PARAM_ST;
	const PARAM_NAME: &'static str = "AutoCreateEnvSoundParam";
}
