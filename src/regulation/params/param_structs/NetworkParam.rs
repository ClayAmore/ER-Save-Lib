use super::{
	defs::NETWORK_PARAM_ST::NETWORK_PARAM_ST,
	param_trait::Param
};
pub struct NetworkParam;
impl Param for NetworkParam {
	type ParamType = NETWORK_PARAM_ST;
	const PARAM_NAME: &'static str = "NetworkParam";
}
