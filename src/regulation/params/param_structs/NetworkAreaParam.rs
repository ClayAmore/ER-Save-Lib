use super::{
	defs::NETWORK_AREA_PARAM_ST::NETWORK_AREA_PARAM_ST,
	param_trait::Param
};
pub struct NetworkAreaParam;
impl Param for NetworkAreaParam {
	type ParamType = NETWORK_AREA_PARAM_ST;
	const PARAM_NAME: &'static str = "NetworkAreaParam";
}
