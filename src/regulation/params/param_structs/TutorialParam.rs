use super::{
	defs::TUTORIAL_PARAM_ST::TUTORIAL_PARAM_ST,
	param_trait::Param
};
pub struct TutorialParam;
impl Param for TutorialParam {
	type ParamType = TUTORIAL_PARAM_ST;
	const PARAM_NAME: &'static str = "TutorialParam";
}
