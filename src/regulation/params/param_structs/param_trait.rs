use deku::ctx::Endian;
use deku::{DekuReader, DekuWriter};
pub trait Param {
	type ParamType: DekuWriter<(Endian, u32)> + for<'a> DekuReader<'a, (Endian, u32)>;
	const PARAM_NAME: &'static str;
}
