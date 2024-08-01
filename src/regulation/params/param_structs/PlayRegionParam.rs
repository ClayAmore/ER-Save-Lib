use super::{
	defs::PLAY_REGION_PARAM_ST::PLAY_REGION_PARAM_ST,
	param_trait::Param
};
pub struct PlayRegionParam;
impl Param for PlayRegionParam {
	type ParamType = PLAY_REGION_PARAM_ST;
	const PARAM_NAME: &'static str = "PlayRegionParam";
}
