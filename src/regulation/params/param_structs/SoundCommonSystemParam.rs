use super::{
	defs::SOUND_COMMON_SYSTEM_PARAM_ST::SOUND_COMMON_SYSTEM_PARAM_ST,
	param_trait::Param
};
pub struct SoundCommonSystemParam;
impl Param for SoundCommonSystemParam {
	type ParamType = SOUND_COMMON_SYSTEM_PARAM_ST;
	const PARAM_NAME: &'static str = "SoundCommonSystemParam";
}
