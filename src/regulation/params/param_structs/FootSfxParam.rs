use super::{
	defs::FOOT_SFX_PARAM_ST::FOOT_SFX_PARAM_ST,
	param_trait::Param
};
pub struct FootSfxParam;
impl Param for FootSfxParam {
	type ParamType = FOOT_SFX_PARAM_ST;
	const PARAM_NAME: &'static str = "FootSfxParam";
}
