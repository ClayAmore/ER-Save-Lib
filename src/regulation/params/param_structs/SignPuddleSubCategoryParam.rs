use super::{
	defs::SIGN_PUDDLE_SUB_CATEGORY_PARAM_ST::SIGN_PUDDLE_SUB_CATEGORY_PARAM_ST,
	param_trait::Param
};
pub struct SignPuddleSubCategoryParam;
impl Param for SignPuddleSubCategoryParam {
	type ParamType = SIGN_PUDDLE_SUB_CATEGORY_PARAM_ST;
	const PARAM_NAME: &'static str = "SignPuddleSubCategoryParam";
}
