use super::{
	defs::WET_ASPECT_PARAM_ST::WET_ASPECT_PARAM_ST,
	param_trait::Param
};
pub struct WetAspectParam;
impl Param for WetAspectParam {
	type ParamType = WET_ASPECT_PARAM_ST;
	const PARAM_NAME: &'static str = "WetAspectParam";
}
