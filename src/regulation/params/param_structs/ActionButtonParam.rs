use super::{
	defs::ACTIONBUTTON_PARAM_ST::ACTIONBUTTON_PARAM_ST,
	param_trait::Param
};
pub struct ActionButtonParam;
impl Param for ActionButtonParam {
	type ParamType = ACTIONBUTTON_PARAM_ST;
	const PARAM_NAME: &'static str = "ActionButtonParam";
}
