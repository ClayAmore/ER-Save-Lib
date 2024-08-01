use super::{
	defs::DECAL_PARAM_ST::DECAL_PARAM_ST,
	param_trait::Param
};
pub struct DecalParam;
impl Param for DecalParam {
	type ParamType = DECAL_PARAM_ST;
	const PARAM_NAME: &'static str = "DecalParam";
}
