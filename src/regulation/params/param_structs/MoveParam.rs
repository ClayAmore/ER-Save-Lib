use super::{
	defs::MOVE_PARAM_ST::MOVE_PARAM_ST,
	param_trait::Param
};
pub struct MoveParam;
impl Param for MoveParam {
	type ParamType = MOVE_PARAM_ST;
	const PARAM_NAME: &'static str = "MoveParam";
}
