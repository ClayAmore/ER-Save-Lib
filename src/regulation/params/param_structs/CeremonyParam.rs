use super::{
	defs::CEREMONY_PARAM_ST::CEREMONY_PARAM_ST,
	param_trait::Param
};
pub struct CeremonyParam;
impl Param for CeremonyParam {
	type ParamType = CEREMONY_PARAM_ST;
	const PARAM_NAME: &'static str = "CeremonyParam";
}
