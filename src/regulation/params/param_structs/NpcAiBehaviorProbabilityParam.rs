use super::{
	defs::NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST::NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST,
	param_trait::Param
};
pub struct NpcAiBehaviorProbabilityParam;
impl Param for NpcAiBehaviorProbabilityParam {
	type ParamType = NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST;
	const PARAM_NAME: &'static str = "NpcAiBehaviorProbabilityParam";
}
