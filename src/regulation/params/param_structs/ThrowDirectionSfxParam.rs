use super::{
	defs::THROW_DIRECTION_SFX_PARAM_ST::THROW_DIRECTION_SFX_PARAM_ST,
	param_trait::Param
};
pub struct ThrowDirectionSfxParam;
impl Param for ThrowDirectionSfxParam {
	type ParamType = THROW_DIRECTION_SFX_PARAM_ST;
	const PARAM_NAME: &'static str = "ThrowDirectionSfxParam";
}
