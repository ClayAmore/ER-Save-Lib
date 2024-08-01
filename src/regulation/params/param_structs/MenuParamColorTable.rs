use super::{
	defs::MENU_PARAM_COLOR_TABLE_ST::MENU_PARAM_COLOR_TABLE_ST,
	param_trait::Param
};
pub struct MenuParamColorTable;
impl Param for MenuParamColorTable {
	type ParamType = MENU_PARAM_COLOR_TABLE_ST;
	const PARAM_NAME: &'static str = "MenuParamColorTable";
}
