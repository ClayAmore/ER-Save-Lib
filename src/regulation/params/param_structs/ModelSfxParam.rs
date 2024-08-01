use super::{
	defs::MODEL_SFX_PARAM_ST::MODEL_SFX_PARAM_ST,
	param_trait::Param
};
pub struct ModelSfxParam;
impl Param for ModelSfxParam {
	type ParamType = MODEL_SFX_PARAM_ST;
	const PARAM_NAME: &'static str = "ModelSfxParam";
}
