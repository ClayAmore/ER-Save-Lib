use super::{
	defs::BUDDY_PARAM_ST::BUDDY_PARAM_ST,
	param_trait::Param
};
pub struct BuddyParam;
impl Param for BuddyParam {
	type ParamType = BUDDY_PARAM_ST;
	const PARAM_NAME: &'static str = "BuddyParam";
}
