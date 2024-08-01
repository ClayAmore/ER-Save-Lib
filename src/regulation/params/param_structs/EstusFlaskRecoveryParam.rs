use super::{
	defs::ESTUS_FLASK_RECOVERY_PARAM_ST::ESTUS_FLASK_RECOVERY_PARAM_ST,
	param_trait::Param
};
pub struct EstusFlaskRecoveryParam;
impl Param for EstusFlaskRecoveryParam {
	type ParamType = ESTUS_FLASK_RECOVERY_PARAM_ST;
	const PARAM_NAME: &'static str = "EstusFlaskRecoveryParam";
}
