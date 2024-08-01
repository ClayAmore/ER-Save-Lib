use super::{
	defs::REINFORCE_PARAM_WEAPON_ST::REINFORCE_PARAM_WEAPON_ST,
	param_trait::Param
};
pub struct ReinforceParamWeapon;
impl Param for ReinforceParamWeapon {
	type ParamType = REINFORCE_PARAM_WEAPON_ST;
	const PARAM_NAME: &'static str = "ReinforceParamWeapon";
}
