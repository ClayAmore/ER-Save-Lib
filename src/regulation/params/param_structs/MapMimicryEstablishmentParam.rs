use super::{
	defs::MAP_MIMICRY_ESTABLISHMENT_PARAM_ST::MAP_MIMICRY_ESTABLISHMENT_PARAM_ST,
	param_trait::Param
};
pub struct MapMimicryEstablishmentParam;
impl Param for MapMimicryEstablishmentParam {
	type ParamType = MAP_MIMICRY_ESTABLISHMENT_PARAM_ST;
	const PARAM_NAME: &'static str = "MapMimicryEstablishmentParam";
}
