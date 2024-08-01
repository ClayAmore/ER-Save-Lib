use super::{
	defs::NPC_PARAM_ST::NPC_PARAM_ST,
	param_trait::Param
};
pub struct NpcParam;
impl Param for NpcParam {
	type ParamType = NPC_PARAM_ST;
	const PARAM_NAME: &'static str = "NpcParam";
}
