use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct NPC_THINK_PARAM_ST {
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	pub disableParamReserve2: [u8;3],
	pub logicId: i32,
	pub battleGoalID: i32,
	pub searchEye_dist: i16,
	pub searchEye_angY: u8,
	#[deku(bits = 5)]
	pub pad8: u8,
	#[deku(bits = 1)]
	pub targetAILockDmyPoly: u8,
	#[deku(bits = 1)]
	pub enableWeaponOnOff: u8,
	#[deku(bits = 1)]
	pub isNoAvoidHugeEnemy: u8,
	pub spEffectId_RangedAttack: i32,
	pub searchTargetLv1ForgetTime: f32,
	pub searchTargetLv2ForgetTime: f32,
	pub BackHomeLife_OnHitEneWal: f32,
	pub SightTargetForgetTime: f32,
	pub idAttackCannotMove: i32,
	pub ear_dist: f32,
	pub callHelp_ActionAnimId: i32,
	pub callHelp_CallActionId: i32,
	pub eye_dist: i16,
	pub isGuard_Act: u8,
	pub pad6: [u8;1],
	pub ear_soundcut_dist: i16,
	pub nose_dist: i16,
	pub maxBackhomeDist: i16,
	pub backhomeDist: i16,
	pub backhomeBattleDist: i16,
	pub nonBattleActLife: i16,
	pub BackHome_LookTargetTime: i16,
	pub BackHome_LookTargetDist: i16,
	pub SoundTargetForgetTime: f32,
	pub BattleStartDist: i16,
	pub callHelp_MyPeerId: i16,
	pub callHelp_CallPeerId: i16,
	pub targetSys_DmgEffectRate: i16,
	pub TeamAttackEffectivity: u8,
	pub eye_angX: u8,
	pub eye_angY: u8,
	pub disableDark: u8,
	pub caravanRole: u8,
	pub callHelp_CallValidMinDistTarget: u8,
	pub callHelp_CallValidRange: u8,
	pub callHelp_ForgetTimeByArrival: u8,
	pub callHelp_MinWaitTime: u8,
	pub callHelp_MaxWaitTime: u8,
	pub goalAction_ToCaution: u8,
	pub ear_listenLevel: u8,
	pub callHelp_ReplyBehaviorType: u8,
	pub disablePathMove: u8,
	pub skipArrivalVisibleCheck: u8,
	pub thinkAttr_doAdmirer: u8,
	#[deku(bits = 1)]
	pub enableNaviFlg_Edge_Ordinary: u8,
	#[deku(bits = 1)]
	pub enableNaviFlg_Lava: u8,
	#[deku(bits = 1)]
	pub enableNaviFlg_InSideWall: u8,
	#[deku(bits = 1)]
	pub enableNaviFlg_Door: u8,
	#[deku(bits = 1)]
	pub enableNaviFlg_Hole: u8,
	#[deku(bits = 1)]
	pub enableNaviFlg_Ladder: u8,
	#[deku(bits = 1)]
	pub enableNaviFlg_LargeSpace: u8,
	#[deku(bits = 1)]
	pub enableNaviFlg_Edge: u8,
	pub enableNaviFlg_reserve1: [u8;3],
	pub searchThreshold_Lv0toLv1: i32,
	pub searchThreshold_Lv1toLv2: i32,
	pub platoonReplyTime: f32,
	pub platoonReplyAddRandomTime: f32,
	pub searchEye_angX: u8,
	pub isUpdateBattleSight: u8,
	pub battleEye_updateDist: i16,
	pub battleEye_updateAngX: u8,
	pub battleEye_updateAngY: u8,
	#[deku(count = "16")]
	pub pad4: Vec<u8>,
	pub eye_BackOffsetDist: i16,
	pub eye_BeginDist: i16,
	pub actTypeOnFailedPath: u8,
	pub goalAction_ToCautionImportant: u8,
	pub shiftAnimeId_RangedAttack: i32,
	pub actTypeOnNonBtlFailedPath: u8,
	pub isBuddyAI: u8,
	pub goalAction_ToSearchLv1: u8,
	pub goalAction_ToSearchLv2: u8,
	pub enableJumpMove: u8,
	pub disableLocalSteering: u8,
	pub goalAction_ToDisappear: u8,
	pub changeStateAction_ToNormal: u8,
	pub MemoryTargetForgetTime: f32,
	pub rangedAttackId: i32,
	pub useFall_onNormalCaution: u8,
	pub useFall_onSearchBattle: u8,
	pub enableJumpMove_onBattle: u8,
	pub backToHomeStuckAct: u8,
	pub pad3: [u8;4],
	pub soundBehaviorId01: i32,
	pub soundBehaviorId02: i32,
	pub soundBehaviorId03: i32,
	pub soundBehaviorId04: i32,
	pub soundBehaviorId05: i32,
	pub soundBehaviorId06: i32,
	pub soundBehaviorId07: i32,
	pub soundBehaviorId08: i32,
	pub weaponOffSpecialEffectId: i32,
	pub weaponOnSpecialEffectId: i32,
	pub weaponOffAnimId: i32,
	pub weaponOnAnimId: i32,
	pub surpriseAnimId: i32,
}
