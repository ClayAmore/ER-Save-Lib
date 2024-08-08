use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct RANDOM_APPEAR_PARAM_ST {
	#[deku(bits = 1)]
	pub slot7: u8,
	#[deku(bits = 1)]
	pub slot6: u8,
	#[deku(bits = 1)]
	pub slot5: u8,
	#[deku(bits = 1)]
	pub slot4: u8,
	#[deku(bits = 1)]
	pub slot3: u8,
	#[deku(bits = 1)]
	pub slot2: u8,
	#[deku(bits = 1)]
	pub slot1: u8,
	#[deku(bits = 1)]
	pub slot0: u8,
	#[deku(bits = 1)]
	pub slot15: u8,
	#[deku(bits = 1)]
	pub slot14: u8,
	#[deku(bits = 1)]
	pub slot13: u8,
	#[deku(bits = 1)]
	pub slot12: u8,
	#[deku(bits = 1)]
	pub slot11: u8,
	#[deku(bits = 1)]
	pub slot10: u8,
	#[deku(bits = 1)]
	pub slot9: u8,
	#[deku(bits = 1)]
	pub slot8: u8,
	#[deku(bits = 1)]
	pub slot23: u8,
	#[deku(bits = 1)]
	pub slot22: u8,
	#[deku(bits = 1)]
	pub slot21: u8,
	#[deku(bits = 1)]
	pub slot20: u8,
	#[deku(bits = 1)]
	pub slot19: u8,
	#[deku(bits = 1)]
	pub slot18: u8,
	#[deku(bits = 1)]
	pub slot17: u8,
	#[deku(bits = 1)]
	pub slot16: u8,
	#[deku(bits = 1)]
	pub slot31: u8,
	#[deku(bits = 1)]
	pub slot30: u8,
	#[deku(bits = 1)]
	pub slot29: u8,
	#[deku(bits = 1)]
	pub slot28: u8,
	#[deku(bits = 1)]
	pub slot27: u8,
	#[deku(bits = 1)]
	pub slot26: u8,
	#[deku(bits = 1)]
	pub slot25: u8,
	#[deku(bits = 1)]
	pub slot24: u8,
	#[deku(bits = 1)]
	pub slot39: u8,
	#[deku(bits = 1)]
	pub slot38: u8,
	#[deku(bits = 1)]
	pub slot37: u8,
	#[deku(bits = 1)]
	pub slot36: u8,
	#[deku(bits = 1)]
	pub slot35: u8,
	#[deku(bits = 1)]
	pub slot34: u8,
	#[deku(bits = 1)]
	pub slot33: u8,
	#[deku(bits = 1)]
	pub slot32: u8,
	#[deku(bits = 1)]
	pub slot47: u8,
	#[deku(bits = 1)]
	pub slot46: u8,
	#[deku(bits = 1)]
	pub slot45: u8,
	#[deku(bits = 1)]
	pub slot44: u8,
	#[deku(bits = 1)]
	pub slot43: u8,
	#[deku(bits = 1)]
	pub slot42: u8,
	#[deku(bits = 1)]
	pub slot41: u8,
	#[deku(bits = 1)]
	pub slot40: u8,
	#[deku(bits = 1)]
	pub slot55: u8,
	#[deku(bits = 1)]
	pub slot54: u8,
	#[deku(bits = 1)]
	pub slot53: u8,
	#[deku(bits = 1)]
	pub slot52: u8,
	#[deku(bits = 1)]
	pub slot51: u8,
	#[deku(bits = 1)]
	pub slot50: u8,
	#[deku(bits = 1)]
	pub slot49: u8,
	#[deku(bits = 1)]
	pub slot48: u8,
	#[deku(bits = 1)]
	pub slot63: u8,
	#[deku(bits = 1)]
	pub slot62: u8,
	#[deku(bits = 1)]
	pub slot61: u8,
	#[deku(bits = 1)]
	pub slot60: u8,
	#[deku(bits = 1)]
	pub slot59: u8,
	#[deku(bits = 1)]
	pub slot58: u8,
	#[deku(bits = 1)]
	pub slot57: u8,
	#[deku(bits = 1)]
	pub slot56: u8,
	#[deku(bits = 1)]
	pub slot71: u8,
	#[deku(bits = 1)]
	pub slot70: u8,
	#[deku(bits = 1)]
	pub slot69: u8,
	#[deku(bits = 1)]
	pub slot68: u8,
	#[deku(bits = 1)]
	pub slot67: u8,
	#[deku(bits = 1)]
	pub slot66: u8,
	#[deku(bits = 1)]
	pub slot65: u8,
	#[deku(bits = 1)]
	pub slot64: u8,
	#[deku(bits = 1)]
	pub slot79: u8,
	#[deku(bits = 1)]
	pub slot78: u8,
	#[deku(bits = 1)]
	pub slot77: u8,
	#[deku(bits = 1)]
	pub slot76: u8,
	#[deku(bits = 1)]
	pub slot75: u8,
	#[deku(bits = 1)]
	pub slot74: u8,
	#[deku(bits = 1)]
	pub slot73: u8,
	#[deku(bits = 1)]
	pub slot72: u8,
	#[deku(bits = 1)]
	pub slot87: u8,
	#[deku(bits = 1)]
	pub slot86: u8,
	#[deku(bits = 1)]
	pub slot85: u8,
	#[deku(bits = 1)]
	pub slot84: u8,
	#[deku(bits = 1)]
	pub slot83: u8,
	#[deku(bits = 1)]
	pub slot82: u8,
	#[deku(bits = 1)]
	pub slot81: u8,
	#[deku(bits = 1)]
	pub slot80: u8,
	#[deku(bits = 1)]
	pub slot95: u8,
	#[deku(bits = 1)]
	pub slot94: u8,
	#[deku(bits = 1)]
	pub slot93: u8,
	#[deku(bits = 1)]
	pub slot92: u8,
	#[deku(bits = 1)]
	pub slot91: u8,
	#[deku(bits = 1)]
	pub slot90: u8,
	#[deku(bits = 1)]
	pub slot89: u8,
	#[deku(bits = 1)]
	pub slot88: u8,
	#[deku(bits = 4)]
	pub pad: u8,
	#[deku(bits = 1)]
	pub slot99: u8,
	#[deku(bits = 1)]
	pub slot98: u8,
	#[deku(bits = 1)]
	pub slot97: u8,
	#[deku(bits = 1)]
	pub slot96: u8,
}
