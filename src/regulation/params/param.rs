use deku::prelude::*;
use deku::{DekuRead, DekuWrite};

use crate::param_trait::Param;

use super::{header::PARAMHeader, row_header::ParamRowHeader};

// PARAM
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(ctx = "version: u32")]
pub(crate) struct PARAM<P: Param> {
    pub(crate) header: PARAMHeader,
    #[deku(
        count = "header.row_count",
        ctx = "header.endian, header.format0x2d, header.format0x2e"
    )]
    pub(crate) row_headers: Vec<ParamRowHeader>,
    #[deku(ctx = "header.endian, version", count = "header.row_count")]
    pub(crate) row_data: Vec<P::ParamType>,
}
