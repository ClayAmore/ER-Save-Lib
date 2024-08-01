use super::{
	defs::SOUND_COMMON_INGAME_PARAM_ST::SOUND_COMMON_INGAME_PARAM_ST,
	param_trait::Param
};
pub struct SoundCommonIngameParam;
impl Param for SoundCommonIngameParam {
	type ParamType = SOUND_COMMON_INGAME_PARAM_ST;
	const PARAM_NAME: &'static str = "SoundCommonIngameParam";
}
