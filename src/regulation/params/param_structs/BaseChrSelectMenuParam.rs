use super::{
	defs::BASECHR_SELECT_MENU_PARAM_ST::BASECHR_SELECT_MENU_PARAM_ST,
	param_trait::Param
};
pub struct BaseChrSelectMenuParam;
impl Param for BaseChrSelectMenuParam {
	type ParamType = BASECHR_SELECT_MENU_PARAM_ST;
	const PARAM_NAME: &'static str = "BaseChrSelectMenuParam";
}
