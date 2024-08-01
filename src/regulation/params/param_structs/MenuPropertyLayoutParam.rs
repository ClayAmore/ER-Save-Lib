use super::{
	defs::MENUPROPERTY_LAYOUT::MENUPROPERTY_LAYOUT,
	param_trait::Param
};
pub struct MenuPropertyLayoutParam;
impl Param for MenuPropertyLayoutParam {
	type ParamType = MENUPROPERTY_LAYOUT;
	const PARAM_NAME: &'static str = "MenuPropertyLayoutParam";
}
