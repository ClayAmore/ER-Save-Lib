use super::{
	defs::CUTSCENE_TEXTURE_LOAD_PARAM_ST::CUTSCENE_TEXTURE_LOAD_PARAM_ST,
	param_trait::Param
};
pub struct CutSceneTextureLoadParam;
impl Param for CutSceneTextureLoadParam {
	type ParamType = CUTSCENE_TEXTURE_LOAD_PARAM_ST;
	const PARAM_NAME: &'static str = "CutSceneTextureLoadParam";
}
