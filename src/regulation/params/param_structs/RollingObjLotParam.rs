use super::{
	defs::ROLLING_OBJ_LOT_PARAM_ST::ROLLING_OBJ_LOT_PARAM_ST,
	param_trait::Param
};
pub struct RollingObjLotParam;
impl Param for RollingObjLotParam {
	type ParamType = ROLLING_OBJ_LOT_PARAM_ST;
	const PARAM_NAME: &'static str = "RollingObjLotParam";
}
