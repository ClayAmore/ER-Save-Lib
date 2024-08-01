use super::{
	defs::PARTS_DRAW_PARAM_ST::PARTS_DRAW_PARAM_ST,
	param_trait::Param
};
pub struct PartsDrawParam;
impl Param for PartsDrawParam {
	type ParamType = PARTS_DRAW_PARAM_ST;
	const PARAM_NAME: &'static str = "PartsDrawParam";
}
