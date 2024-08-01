use super::{
	defs::ROLE_PARAM_ST::ROLE_PARAM_ST,
	param_trait::Param
};
pub struct RoleParam;
impl Param for RoleParam {
	type ParamType = ROLE_PARAM_ST;
	const PARAM_NAME: &'static str = "RoleParam";
}
