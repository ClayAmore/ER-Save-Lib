use super::{
	defs::CACL_CORRECT_GRAPH_ST::CACL_CORRECT_GRAPH_ST,
	param_trait::Param
};
pub struct CalcCorrectGraph;
impl Param for CalcCorrectGraph {
	type ParamType = CACL_CORRECT_GRAPH_ST;
	const PARAM_NAME: &'static str = "CalcCorrectGraph";
}
