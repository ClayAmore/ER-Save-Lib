use super::{
	defs::DEFAULT_KEY_ASSIGN::DEFAULT_KEY_ASSIGN,
	param_trait::Param
};
pub struct DefaultKeyAssign;
impl Param for DefaultKeyAssign {
	type ParamType = DEFAULT_KEY_ASSIGN;
	const PARAM_NAME: &'static str = "DefaultKeyAssign";
}
