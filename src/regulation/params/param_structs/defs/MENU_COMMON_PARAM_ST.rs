use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MENU_COMMON_PARAM_ST {
	pub soloPlayDeath_ToFadeOutTime: f32,
	pub partyGhostDeath_ToFadeOutTime: f32,
	pub playerMaxHpLimit: i32,
	pub playerMaxMpLimit: i32,
	pub playerMaxSpLimit: i32,
	pub actionPanelChangeThreshold_Vel: f32,
	pub actionPanelChangeThreshold_PassTime: f32,
	pub kgIconVspace: i32,
	pub worldMapCursorSelectRadius: f32,
	pub reserved8: [u8;4],
	pub decalPosOffsetX: i32,
	pub decalPosOffsetY: i32,
	pub targetStateSearchDurationTime: f32,
	pub targetStateBattleDurationTime: f32,
	pub worldMapCursorSpeed: f32,
	pub worldMapCursorFirstDistance: f32,
	pub worldMapCursorFirstDelay: f32,
	pub worldMapCursorWaitTime: f32,
	pub worldMapCursorSnapRadius: f32,
	pub worldMapCursorSnapTime: f32,
	pub itemGetLogAliveTime: f32,
	pub playerMaxSaLimit: i32,
	pub worldMap_IsChangeableLayerEventFlagId: i32,
	pub worldMap_TravelMargin: f32,
	pub systemAnnounceScrollBufferTime: f32,
	pub systemAnnounceScrollSpeed: i32,
	pub systemAnnounceNoScrollWaitTime: f32,
	pub systemAnnounceScrollCount: u8,
	pub reserved17: [u8;3],
	pub compassMemoDispDistance: f32,
	pub compassBonfireDispDistance: f32,
	pub markerGoalThreshold: f32,
	pub svSliderStep: f32,
	pub preOpeningMovie_WaitSec: f32,
	pub kgIconScale: f32,
	pub kgIconScale_forTable: f32,
	pub kgIconVspace_forTable: i32,
	pub kgIconScale_forConfig: f32,
	pub kgIconVspace_forConfig: i32,
	pub worldMap_SearchRadius: f32,
	pub tutorialDisplayTime: f32,
	pub compassFriendHostInnerDistance: f32,
	pub compassEnemyHostInnerDistance: f32,
	pub compassFriendGuestInnerDistance: f32,
	pub cutsceneKeyGuideAliveTime: f32,
	pub autoHideHpThresholdRatio: f32,
	pub autoHideHpThresholdValue: i32,
	pub autoHideMpThresholdRatio: f32,
	pub autoHideMpThresholdValue: i32,
	pub autoHideSpThresholdRatio: f32,
	pub autoHideSpThresholdValue: i32,
	pub worldMapZoomAnimationTime: f32,
	pub worldMapIconScaleMin: f32,
	pub worldMap_TravelMargin_Point: f32,
	pub enemyTagSafeLeft: i16,
	pub enemyTagSafeRight: i16,
	pub enemyTagSafeTop: i16,
	pub enemyTagSafeBottom: i16,
	pub pcHorseHpRecoverDispThreshold: i32,
	#[deku(skip, cond = "version >= 11210015", count = "32")]
	pub reserved33_old: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe0: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe1: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe2: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe3: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe4: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe5: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe6: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe7: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe8: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xe9: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xea: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xeb: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xec: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xed: u8,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xee: u8,
	#[deku(skip, cond = "version < 11210015", count = "17")]
	pub reserved33: Vec<u8>,
}
