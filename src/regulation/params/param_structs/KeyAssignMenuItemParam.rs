use super::{
	defs::CS_KEY_ASSIGN_MENUITEM_PARAM::CS_KEY_ASSIGN_MENUITEM_PARAM,
	param_trait::Param
};
pub struct KeyAssignMenuItemParam;
impl Param for KeyAssignMenuItemParam {
	type ParamType = CS_KEY_ASSIGN_MENUITEM_PARAM;
	const PARAM_NAME: &'static str = "KeyAssignMenuItemParam";
}
