use super::{
	defs::BUDGET_PARAM_ST::BUDGET_PARAM_ST,
	param_trait::Param
};
pub struct BudgetParam;
impl Param for BudgetParam {
	type ParamType = BUDGET_PARAM_ST;
	const PARAM_NAME: &'static str = "BudgetParam";
}
