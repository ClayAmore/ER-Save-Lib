use super::{
	defs::MENU_COMMON_PARAM_ST::MENU_COMMON_PARAM_ST,
	param_trait::Param
};
pub struct MenuCommonParam;
impl Param for MenuCommonParam {
	type ParamType = MENU_COMMON_PARAM_ST;
	const PARAM_NAME: &'static str = "MenuCommonParam";
}
