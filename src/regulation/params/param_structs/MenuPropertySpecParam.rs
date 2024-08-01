use super::{
	defs::MENUPROPERTY_SPEC::MENUPROPERTY_SPEC,
	param_trait::Param
};
pub struct MenuPropertySpecParam;
impl Param for MenuPropertySpecParam {
	type ParamType = MENUPROPERTY_SPEC;
	const PARAM_NAME: &'static str = "MenuPropertySpecParam";
}
