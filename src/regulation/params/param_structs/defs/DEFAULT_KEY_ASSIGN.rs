use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct DEFAULT_KEY_ASSIGN {
	#[deku(bits = 1)]
	pub priority7: u8,
	#[deku(bits = 1)]
	pub priority6: u8,
	#[deku(bits = 1)]
	pub priority5: u8,
	#[deku(bits = 1)]
	pub priority4: u8,
	#[deku(bits = 1)]
	pub priority3: u8,
	#[deku(bits = 1)]
	pub priority2: u8,
	#[deku(bits = 1)]
	pub priority1: u8,
	#[deku(bits = 1)]
	pub priority0: u8,
	#[deku(bits = 1)]
	pub priority15: u8,
	#[deku(bits = 1)]
	pub priority14: u8,
	#[deku(bits = 1)]
	pub priority13: u8,
	#[deku(bits = 1)]
	pub priority12: u8,
	#[deku(bits = 1)]
	pub priority11: u8,
	#[deku(bits = 1)]
	pub priority10: u8,
	#[deku(bits = 1)]
	pub priority9: u8,
	#[deku(bits = 1)]
	pub priority8: u8,
	#[deku(bits = 1)]
	pub priority23: u8,
	#[deku(bits = 1)]
	pub priority22: u8,
	#[deku(bits = 1)]
	pub priority21: u8,
	#[deku(bits = 1)]
	pub priority20: u8,
	#[deku(bits = 1)]
	pub priority19: u8,
	#[deku(bits = 1)]
	pub priority18: u8,
	#[deku(bits = 1)]
	pub priority17: u8,
	#[deku(bits = 1)]
	pub priority16: u8,
	#[deku(bits = 1)]
	pub priority31: u8,
	#[deku(bits = 1)]
	pub priority30: u8,
	#[deku(bits = 1)]
	pub priority29: u8,
	#[deku(bits = 1)]
	pub priority28: u8,
	#[deku(bits = 1)]
	pub priority27: u8,
	#[deku(bits = 1)]
	pub priority26: u8,
	#[deku(bits = 1)]
	pub priority25: u8,
	#[deku(bits = 1)]
	pub priority24: u8,
	#[deku(count = "12")]
	pub dummy: Vec<u8>,
	pub phyisicalKey_0: i32,
	pub traitsType_0: u8,
	pub a2dOperator_0: u8,
	pub applyTarget_0: u8,
	#[deku(bits = 1)]
	pub enableXboxOne_0: u8,
	#[deku(bits = 1)]
	pub enablePS4_0: u8,
	#[deku(bits = 1)]
	pub enableWin64_0: u8,
	#[deku(bits = 1)]
	pub isAnalog_0: u8,
	pub time1_0: f32,
	pub time2_0: f32,
	pub a2dThreshold_0: f32,
	pub phyisicalKey_1: i32,
	pub traitsType_1: u8,
	pub a2dOperator_1: u8,
	pub applyTarget_1: u8,
	#[deku(bits = 1)]
	pub enableXboxOne_1: u8,
	#[deku(bits = 1)]
	pub enablePS4_1: u8,
	#[deku(bits = 1)]
	pub enableWin64_1: u8,
	#[deku(bits = 1)]
	pub isAnalog_1: u8,
	pub time1_1: f32,
	pub time2_1: f32,
	pub a2dThreshold_1: f32,
	pub phyisicalKey_2: i32,
	pub traitsType_2: u8,
	pub a2dOperator_2: u8,
	pub applyTarget_2: u8,
	#[deku(bits = 1)]
	pub enableXboxOne_2: u8,
	#[deku(bits = 1)]
	pub enablePS4_2: u8,
	#[deku(bits = 1)]
	pub enableWin64_2: u8,
	#[deku(bits = 1)]
	pub isAnalog_2: u8,
	pub time1_2: f32,
	pub time2_2: f32,
	pub a2dThreshold_2: f32,
	pub phyisicalKey_3: i32,
	pub traitsType_3: u8,
	pub a2dOperator_3: u8,
	pub applyTarget_3: u8,
	#[deku(bits = 1)]
	pub enableXboxOne_3: u8,
	#[deku(bits = 1)]
	pub enablePS4_3: u8,
	#[deku(bits = 1)]
	pub enableWin64_3: u8,
	#[deku(bits = 1)]
	pub isAnalog_3: u8,
	pub time1_3: f32,
	pub time2_3: f32,
	pub a2dThreshold_3: f32,
	pub phyisicalKey_4: i32,
	pub traitsType_4: u8,
	pub a2dOperator_4: u8,
	pub applyTarget_4: u8,
	#[deku(bits = 1)]
	pub enableXboxOne_4: u8,
	#[deku(bits = 1)]
	pub enablePS4_4: u8,
	#[deku(bits = 1)]
	pub enableWin64_4: u8,
	#[deku(bits = 1)]
	pub isAnalog_4: u8,
	pub time1_4: f32,
	pub time2_4: f32,
	pub a2dThreshold_4: f32,
	pub phyisicalKey_5: i32,
	pub traitsType_5: u8,
	pub a2dOperator_5: u8,
	pub applyTarget_5: u8,
	#[deku(bits = 1)]
	pub enableXboxOne_5: u8,
	#[deku(bits = 1)]
	pub enablePS4_5: u8,
	#[deku(bits = 1)]
	pub enableWin64_5: u8,
	#[deku(bits = 1)]
	pub isAnalog_5: u8,
	pub time1_5: f32,
	pub time2_5: f32,
	pub a2dThreshold_5: f32,
	pub phyisicalKey_6: i32,
	pub traitsType_6: u8,
	pub a2dOperator_6: u8,
	pub applyTarget_6: u8,
	#[deku(bits = 1)]
	pub enableXboxOne_6: u8,
	#[deku(bits = 1)]
	pub enablePS4_6: u8,
	#[deku(bits = 1)]
	pub enableWin64_6: u8,
	#[deku(bits = 1)]
	pub isAnalog_6: u8,
	pub time1_6: f32,
	pub time2_6: f32,
	pub a2dThreshold_6: f32,
	pub phyisicalKey_7: i32,
	pub traitsType_7: u8,
	pub a2dOperator_7: u8,
	pub applyTarget_7: u8,
	#[deku(bits = 1)]
	pub enableXboxOne_7: u8,
	#[deku(bits = 1)]
	pub enablePS4_7: u8,
	#[deku(bits = 1)]
	pub enableWin64_7: u8,
	#[deku(bits = 1)]
	pub isAnalog_7: u8,
	pub time1_7: f32,
	pub time2_7: f32,
	pub a2dThreshold_7: f32,
}
