use super::{
	defs::LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM::LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM,
	param_trait::Param
};
pub struct LegacyDistantViewPartsReplaceParam;
impl Param for LegacyDistantViewPartsReplaceParam {
	type ParamType = LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM;
	const PARAM_NAME: &'static str = "LegacyDistantViewPartsReplaceParam";
}
