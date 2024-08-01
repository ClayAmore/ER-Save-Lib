use super::{
	defs::ENV_OBJ_LOT_PARAM_ST::ENV_OBJ_LOT_PARAM_ST,
	param_trait::Param
};
pub struct EnvObjLotParam;
impl Param for EnvObjLotParam {
	type ParamType = ENV_OBJ_LOT_PARAM_ST;
	const PARAM_NAME: &'static str = "EnvObjLotParam";
}
