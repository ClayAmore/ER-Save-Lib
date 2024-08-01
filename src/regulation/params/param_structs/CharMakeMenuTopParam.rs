use super::{
	defs::CHARMAKEMENUTOP_PARAM_ST::CHARMAKEMENUTOP_PARAM_ST,
	param_trait::Param
};
pub struct CharMakeMenuTopParam;
impl Param for CharMakeMenuTopParam {
	type ParamType = CHARMAKEMENUTOP_PARAM_ST;
	const PARAM_NAME: &'static str = "CharMakeMenuTopParam";
}
