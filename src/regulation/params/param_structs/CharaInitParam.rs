use super::{
	defs::CHARACTER_INIT_PARAM::CHARACTER_INIT_PARAM,
	param_trait::Param
};
pub struct CharaInitParam;
impl Param for CharaInitParam {
	type ParamType = CHARACTER_INIT_PARAM;
	const PARAM_NAME: &'static str = "CharaInitParam";
}
