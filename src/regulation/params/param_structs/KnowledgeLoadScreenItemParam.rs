use super::{
	defs::KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST::KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST,
	param_trait::Param
};
pub struct KnowledgeLoadScreenItemParam;
impl Param for KnowledgeLoadScreenItemParam {
	type ParamType = KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST;
	const PARAM_NAME: &'static str = "KnowledgeLoadScreenItemParam";
}
