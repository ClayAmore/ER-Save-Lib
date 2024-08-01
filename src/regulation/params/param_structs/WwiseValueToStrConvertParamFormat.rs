use super::{
	defs::WWISE_VALUE_TO_STR_CONVERT_PARAM_ST::WWISE_VALUE_TO_STR_CONVERT_PARAM_ST,
	param_trait::Param
};
pub struct WwiseValueToStrConvertParamFormat;
impl Param for WwiseValueToStrConvertParamFormat {
	type ParamType = WWISE_VALUE_TO_STR_CONVERT_PARAM_ST;
	const PARAM_NAME: &'static str = "WwiseValueToStrConvertParamFormat";
}
