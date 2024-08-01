use super::{
	defs::SFX_BLOCK_RES_SHARE_PARAM::SFX_BLOCK_RES_SHARE_PARAM,
	param_trait::Param
};
pub struct SfxBlockResShareParam;
impl Param for SfxBlockResShareParam {
	type ParamType = SFX_BLOCK_RES_SHARE_PARAM;
	const PARAM_NAME: &'static str = "SfxBlockResShareParam";
}
