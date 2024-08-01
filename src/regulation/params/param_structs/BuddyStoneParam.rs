use super::{
	defs::BUDDY_STONE_PARAM_ST::BUDDY_STONE_PARAM_ST,
	param_trait::Param
};
pub struct BuddyStoneParam;
impl Param for BuddyStoneParam {
	type ParamType = BUDDY_STONE_PARAM_ST;
	const PARAM_NAME: &'static str = "BuddyStoneParam";
}
