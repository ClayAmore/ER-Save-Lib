use super::{
	defs::NETWORK_MSG_PARAM_ST::NETWORK_MSG_PARAM_ST,
	param_trait::Param
};
pub struct NetworkMsgParam;
impl Param for NetworkMsgParam {
	type ParamType = NETWORK_MSG_PARAM_ST;
	const PARAM_NAME: &'static str = "NetworkMsgParam";
}
