use super::{
	defs::CHARMAKEMENU_LISTITEM_PARAM_ST::CHARMAKEMENU_LISTITEM_PARAM_ST,
	param_trait::Param
};
pub struct CharMakeMenuListItemParam;
impl Param for CharMakeMenuListItemParam {
	type ParamType = CHARMAKEMENU_LISTITEM_PARAM_ST;
	const PARAM_NAME: &'static str = "CharMakeMenuListItemParam";
}
