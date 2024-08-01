use super::{
	defs::HIT_MTRL_PARAM_ST::HIT_MTRL_PARAM_ST,
	param_trait::Param
};
pub struct HitMtrlParam;
impl Param for HitMtrlParam {
	type ParamType = HIT_MTRL_PARAM_ST;
	const PARAM_NAME: &'static str = "HitMtrlParam";
}
