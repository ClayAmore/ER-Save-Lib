use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct ROLE_PARAM_ST {
	pub teamType: u8,
	pub pad10: [u8;3],
	pub phantomParamId: i32,
	pub spEffectID0: i32,
	pub spEffectID1: i32,
	pub spEffectID2: i32,
	pub spEffectID3: i32,
	pub spEffectID4: i32,
	pub spEffectID5: i32,
	pub spEffectID6: i32,
	pub spEffectID7: i32,
	pub spEffectID8: i32,
	pub spEffectID9: i32,
	pub sosSignSfxId: i32,
	pub mySosSignSfxId: i32,
	pub summonStartAnimId: i32,
	pub itemlotParamId: i32,
	pub voiceChatGroup: u8,
	pub roleNameColor: u8,
	pub pad1: [u8;2],
	pub roleNameId: i32,
	pub threatLv: i32,
	pub phantomParamId_vowRank1: i32,
	pub phantomParamId_vowRank2: i32,
	pub phantomParamId_vowRank3: i32,
	pub spEffectID_vowRank0: i32,
	pub spEffectID_vowRank1: i32,
	pub spEffectID_vowRank2: i32,
	pub spEffectID_vowRank3: i32,
	pub signPhantomId: i32,
	pub nonPlayerSummonStartAnimId: i32,
	#[deku(count = "16")]
	pub pad2: Vec<u8>,
}
