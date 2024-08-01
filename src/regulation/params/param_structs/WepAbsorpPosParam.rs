use super::{
	defs::WEP_ABSORP_POS_PARAM_ST::WEP_ABSORP_POS_PARAM_ST,
	param_trait::Param
};
pub struct WepAbsorpPosParam;
impl Param for WepAbsorpPosParam {
	type ParamType = WEP_ABSORP_POS_PARAM_ST;
	const PARAM_NAME: &'static str = "WepAbsorpPosParam";
}
