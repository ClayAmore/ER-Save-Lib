use super::{
	defs::ASSET_GEOMETORY_PARAM_ST::ASSET_GEOMETORY_PARAM_ST,
	param_trait::Param
};
pub struct AssetGeometryParam;
impl Param for AssetGeometryParam {
	type ParamType = ASSET_GEOMETORY_PARAM_ST;
	const PARAM_NAME: &'static str = "AssetGeometryParam";
}
