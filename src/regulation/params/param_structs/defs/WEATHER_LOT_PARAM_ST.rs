use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct WEATHER_LOT_PARAM_ST {
	#[deku(bits = 1)]
	pub disableParam_NT: u8,
	#[deku(bits = 7)]
	pub disableParamReserve1: u8,
	pub disableParamReserve2: [u8;3],
	pub weatherType0: i16,
	pub weatherType1: i16,
	pub weatherType2: i16,
	pub weatherType3: i16,
	pub weatherType4: i16,
	pub weatherType5: i16,
	pub weatherType6: i16,
	pub weatherType7: i16,
	pub weatherType8: i16,
	pub weatherType9: i16,
	pub weatherType10: i16,
	pub weatherType11: i16,
	pub weatherType12: i16,
	pub weatherType13: i16,
	pub weatherType14: i16,
	pub weatherType15: i16,
	pub lotteryWeight0: i16,
	pub lotteryWeight1: i16,
	pub lotteryWeight2: i16,
	pub lotteryWeight3: i16,
	pub lotteryWeight4: i16,
	pub lotteryWeight5: i16,
	pub lotteryWeight6: i16,
	pub lotteryWeight7: i16,
	pub lotteryWeight8: i16,
	pub lotteryWeight9: i16,
	pub lotteryWeight10: i16,
	pub lotteryWeight11: i16,
	pub lotteryWeight12: i16,
	pub lotteryWeight13: i16,
	pub lotteryWeight14: i16,
	pub lotteryWeight15: i16,
	pub timezoneLimit: u8,
	pub timezoneStartHour: u8,
	pub timezoneStartMinute: u8,
	pub timezoneEndHour: u8,
	pub timezoneEndMinute: u8,
	#[deku(count = "9")]
	pub reserve: Vec<u8>,
}
