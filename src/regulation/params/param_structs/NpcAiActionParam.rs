use super::{
	defs::NPC_AI_ACTION_PARAM_ST::NPC_AI_ACTION_PARAM_ST,
	param_trait::Param
};
pub struct NpcAiActionParam;
impl Param for NpcAiActionParam {
	type ParamType = NPC_AI_ACTION_PARAM_ST;
	const PARAM_NAME: &'static str = "NpcAiActionParam";
}
