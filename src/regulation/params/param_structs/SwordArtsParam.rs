use super::{
	defs::SWORD_ARTS_PARAM_ST::SWORD_ARTS_PARAM_ST,
	param_trait::Param
};
pub struct SwordArtsParam;
impl Param for SwordArtsParam {
	type ParamType = SWORD_ARTS_PARAM_ST;
	const PARAM_NAME: &'static str = "SwordArtsParam";
}
