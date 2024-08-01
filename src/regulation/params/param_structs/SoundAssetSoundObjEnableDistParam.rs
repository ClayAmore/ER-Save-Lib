use super::{
	defs::SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST::SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST,
	param_trait::Param
};
pub struct SoundAssetSoundObjEnableDistParam;
impl Param for SoundAssetSoundObjEnableDistParam {
	type ParamType = SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST;
	const PARAM_NAME: &'static str = "SoundAssetSoundObjEnableDistParam";
}
