use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct REVERB_AUX_SEND_BUS_PARAM_ST {
	#[deku(count = "32")]
	pub ReverbAuxSendBusName: Vec<u8>,
}
