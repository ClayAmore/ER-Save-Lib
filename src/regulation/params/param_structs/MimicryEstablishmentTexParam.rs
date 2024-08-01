use super::{
	defs::MIMICRY_ESTABLISHMENT_TEX_PARAM_ST::MIMICRY_ESTABLISHMENT_TEX_PARAM_ST,
	param_trait::Param
};
pub struct MimicryEstablishmentTexParam;
impl Param for MimicryEstablishmentTexParam {
	type ParamType = MIMICRY_ESTABLISHMENT_TEX_PARAM_ST;
	const PARAM_NAME: &'static str = "MimicryEstablishmentTexParam";
}
