use super::{
	defs::ASSET_MATERIAL_SFX_PARAM_ST::ASSET_MATERIAL_SFX_PARAM_ST,
	param_trait::Param
};
pub struct AssetMaterialSfxParam;
impl Param for AssetMaterialSfxParam {
	type ParamType = ASSET_MATERIAL_SFX_PARAM_ST;
	const PARAM_NAME: &'static str = "AssetMaterialSfxParam";
}
