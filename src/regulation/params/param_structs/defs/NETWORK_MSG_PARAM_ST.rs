use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct NETWORK_MSG_PARAM_ST {
	pub priority: i16,
	pub forcePlay: u8,
	pub pad1: [u8;1],
	pub normalWhite: i32,
	pub umbasaWhite: i32,
	pub berserkerWhite: i32,
	pub sinnerHeroWhite: i32,
	pub normalBlack: i32,
	pub umbasaBlack: i32,
	pub berserkerBlack: i32,
	pub forceJoinBlack: i32,
	pub forceJoinUmbasaBlack: i32,
	pub forceJoinBerserkerBlack: i32,
	pub sinnerHunterVisitor: i32,
	pub redHunterVisitor: i32,
	pub guardianOfBossVisitor: i32,
	pub guardianOfForestMapVisitor: i32,
	pub guardianOfAnolisVisitor: i32,
	pub rosaliaBlack: i32,
	pub forceJoinRosaliaBlack: i32,
	pub redHunterVisitor2: i32,
	pub npc1: i32,
	pub npc2: i32,
	pub npc3: i32,
	pub npc4: i32,
	pub battleRoyal: i32,
	pub npc5: i32,
	pub npc6: i32,
	pub npc7: i32,
	pub npc8: i32,
	pub npc9: i32,
	pub npc10: i32,
	pub npc11: i32,
	pub npc12: i32,
	pub npc13: i32,
	pub npc14: i32,
	pub npc15: i32,
	pub npc16: i32,
	pub forceJoinBlack_B: i32,
	pub normalWhite_Npc: i32,
	pub forceJoinBlack_Npc: i32,
	pub forceJoinBlack_B_Npc: i32,
	pub forceJoinBlack_C_Npc: i32,
	#[deku(skip, cond = "version >= 11210015", count = "28")]
	pub pad2: Vec<u8>,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xa4: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xa8: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xac: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xb0: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub unknown_0xb4: i32,
	#[deku(skip, cond = "version < 11210015")]
	pub pad2_new: [u8;8],
}
