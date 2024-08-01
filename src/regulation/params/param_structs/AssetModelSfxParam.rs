use super::{
	defs::ASSET_MODEL_SFX_PARAM_ST::ASSET_MODEL_SFX_PARAM_ST,
	param_trait::Param
};
pub struct AssetModelSfxParam;
impl Param for AssetModelSfxParam {
	type ParamType = ASSET_MODEL_SFX_PARAM_ST;
	const PARAM_NAME: &'static str = "AssetModelSfxParam";
}
