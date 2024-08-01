use super::{
	defs::RIDE_PARAM_ST::RIDE_PARAM_ST,
	param_trait::Param
};
pub struct RideParam;
impl Param for RideParam {
	type ParamType = RIDE_PARAM_ST;
	const PARAM_NAME: &'static str = "RideParam";
}
