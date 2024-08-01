use super::{
	defs::SPEEDTREE_MODEL_PARAM_ST::SPEEDTREE_MODEL_PARAM_ST,
	param_trait::Param
};
pub struct SpeedtreeModel;
impl Param for SpeedtreeModel {
	type ParamType = SPEEDTREE_MODEL_PARAM_ST;
	const PARAM_NAME: &'static str = "SpeedtreeModel";
}
