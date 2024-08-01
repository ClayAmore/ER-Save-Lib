use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct ENEMY_STANDARD_INFO_BANK {
	pub EnemyBehaviorID: i32,
	pub HP: i16,
	pub AttackPower: i16,
	pub ChrType: i32,
	pub HitHeight: f32,
	pub HitRadius: f32,
	pub Weight: f32,
	pub DynamicFriction: f32,
	pub StaticFriction: f32,
	pub UpperDefState: i32,
	pub ActionDefState: i32,
	pub RotY_per_Second: f32,
	#[deku(count = "20")]
	pub reserve0: Vec<u8>,
	pub RotY_per_Second_old: u8,
	pub EnableSideStep: u8,
	pub UseRagdollHit: u8,
	pub reserve_last: [u8;5],
	pub stamina: i16,
	pub staminaRecover: i16,
	pub staminaConsumption: i16,
	pub deffenct_Phys: i16,
	#[deku(count = "48")]
	pub reserve_last2: Vec<u8>,
}
