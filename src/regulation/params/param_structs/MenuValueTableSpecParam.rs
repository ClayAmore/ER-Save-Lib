use super::{
	defs::MENU_VALUE_TABLE_SPEC::MENU_VALUE_TABLE_SPEC,
	param_trait::Param
};
pub struct MenuValueTableSpecParam;
impl Param for MenuValueTableSpecParam {
	type ParamType = MENU_VALUE_TABLE_SPEC;
	const PARAM_NAME: &'static str = "MenuValueTableSpecParam";
}
