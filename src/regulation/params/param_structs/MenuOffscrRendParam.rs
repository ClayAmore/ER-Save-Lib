use super::{
	defs::MENU_OFFSCR_REND_PARAM_ST::MENU_OFFSCR_REND_PARAM_ST,
	param_trait::Param
};
pub struct MenuOffscrRendParam;
impl Param for MenuOffscrRendParam {
	type ParamType = MENU_OFFSCR_REND_PARAM_ST;
	const PARAM_NAME: &'static str = "MenuOffscrRendParam";
}
