use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct MISSILE_PARAM_ST {
	pub FFXID: i32,
	pub LifeTime: i16,
	pub HitSphereRadius: i16,
	pub HitDamage: i16,
	pub reserve0: [u8;6],
	pub InitVelocity: f32,
	pub distance: f32,
	pub gravityInRange: f32,
	pub gravityOutRange: f32,
	pub mp: i32,
	pub accelInRange: f32,
	pub accelOutRange: f32,
	#[deku(count = "20")]
	pub reserve1: Vec<u8>,
	pub HitMissileID: i16,
	pub DiedNaturaly: u8,
	pub ExplosionDie: u8,
	pub behaviorId: i32,
	#[deku(count = "56")]
	pub reserve_last: Vec<u8>,
}
