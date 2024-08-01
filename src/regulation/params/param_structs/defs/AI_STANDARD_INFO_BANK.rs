use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct AI_STANDARD_INFO_BANK {
	pub RadarRange: i16,
	pub RadarAngleX: u8,
	pub RadarAngleY: u8,
	pub TerritorySize: i16,
	pub ThreatBeforeAttackRate: u8,
	pub ForceThreatOnFirstLocked: u8,
	#[deku(count = "24")]
	pub reserve0: Vec<u8>,
	pub Attack1_Distance: i16,
	pub Attack1_Margin: i16,
	pub Attack1_Rate: u8,
	pub Attack1_ActionID: u8,
	pub Attack1_DelayMin: u8,
	pub Attack1_DelayMax: u8,
	pub Attack1_ConeAngle: u8,
	pub reserve10: [u8;7],
	pub Attack2_Distance: i16,
	pub Attack2_Margin: i16,
	pub Attack2_Rate: u8,
	pub Attack2_ActionID: u8,
	pub Attack2_DelayMin: u8,
	pub Attack2_DelayMax: u8,
	pub Attack2_ConeAngle: u8,
	pub reserve11: [u8;7],
	pub Attack3_Distance: i16,
	pub Attack3_Margin: i16,
	pub Attack3_Rate: u8,
	pub Attack3_ActionID: u8,
	pub Attack3_DelayMin: u8,
	pub Attack3_DelayMax: u8,
	pub Attack3_ConeAngle: u8,
	pub reserve12: [u8;7],
	pub Attack4_Distance: i16,
	pub Attack4_Margin: i16,
	pub Attack4_Rate: u8,
	pub Attack4_ActionID: u8,
	pub Attack4_DelayMin: u8,
	pub Attack4_DelayMax: u8,
	pub Attack4_ConeAngle: u8,
	pub reserve13: [u8;7],
	#[deku(count = "32")]
	pub reserve_last: Vec<u8>,
}
