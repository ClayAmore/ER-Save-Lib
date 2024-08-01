use super::{
	defs::REVERB_AUX_SEND_BUS_PARAM_ST::REVERB_AUX_SEND_BUS_PARAM_ST,
	param_trait::Param
};
pub struct ReverbAuxSendBusParam;
impl Param for ReverbAuxSendBusParam {
	type ParamType = REVERB_AUX_SEND_BUS_PARAM_ST;
	const PARAM_NAME: &'static str = "ReverbAuxSendBusParam";
}
