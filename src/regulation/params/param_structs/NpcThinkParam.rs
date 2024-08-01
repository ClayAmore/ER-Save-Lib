use super::{
	defs::NPC_THINK_PARAM_ST::NPC_THINK_PARAM_ST,
	param_trait::Param
};
pub struct NpcThinkParam;
impl Param for NpcThinkParam {
	type ParamType = NPC_THINK_PARAM_ST;
	const PARAM_NAME: &'static str = "NpcThinkParam";
}
