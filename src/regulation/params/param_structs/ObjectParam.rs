use super::{
	defs::OBJECT_PARAM_ST::OBJECT_PARAM_ST,
	param_trait::Param
};
pub struct ObjectParam;
impl Param for ObjectParam {
	type ParamType = OBJECT_PARAM_ST;
	const PARAM_NAME: &'static str = "ObjectParam";
}
