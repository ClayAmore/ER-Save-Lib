use super::{
	defs::GPARAM_REF_SETTINGS_PARAM_ST::GPARAM_REF_SETTINGS_PARAM_ST,
	param_trait::Param
};
pub struct GparamRefSettingsParam;
impl Param for GparamRefSettingsParam {
	type ParamType = GPARAM_REF_SETTINGS_PARAM_ST;
	const PARAM_NAME: &'static str = "GparamRefSettingsParam";
}
