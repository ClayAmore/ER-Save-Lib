use super::{
	defs::SOUND_CHR_PHYSICS_SE_PARAM_ST::SOUND_CHR_PHYSICS_SE_PARAM_ST,
	param_trait::Param
};
pub struct SoundChrPhysicsSeParam;
impl Param for SoundChrPhysicsSeParam {
	type ParamType = SOUND_CHR_PHYSICS_SE_PARAM_ST;
	const PARAM_NAME: &'static str = "SoundChrPhysicsSeParam";
}
