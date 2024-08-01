use super::{
	defs::OBJ_ACT_PARAM_ST::OBJ_ACT_PARAM_ST,
	param_trait::Param
};
pub struct ObjActParam;
impl Param for ObjActParam {
	type ParamType = OBJ_ACT_PARAM_ST;
	const PARAM_NAME: &'static str = "ObjActParam";
}
