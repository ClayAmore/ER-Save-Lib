use std::borrow::Cow;
use std::collections::BTreeMap;

use deku::prelude::*;

use crate::regulation::bnd4::{file_header::BND4FileHeader, header::BND4Header};

#[derive(PartialEq, Debug)]
pub(crate) enum ParamType {
    String(String),
    Offset(i64),
}

#[derive(PartialEq, Debug)]
pub(crate) enum Offset {
    None,
    Long(i64),
    Int(i32),
}

// Context params for Params
type Ctx<'a> = (&'a BND4Header, &'a Vec<BND4FileHeader>, &'a Vec<String>);

#[derive(PartialEq, Debug)]
pub(crate) struct Params {
    pub(crate) param_files: BTreeMap<String, Vec<u8>>,
}
impl<'a> DekuReader<'a, Ctx<'a>> for Params {
    fn from_reader_with_ctx<R: std::io::Read>(
        reader: &mut Reader<R>,
        ctx: Ctx<'a>,
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let (_, file_headers, file_names) = ctx;

        let mut param_files = BTreeMap::new();
        for (file_header, file_name) in file_headers.iter().zip(file_names) {
            let bytes_read = reader.bits_read / 8;
            let data_offset = file_header.data_offset as usize;
            let compressed_size = file_header.compressed_size as usize;

            let file_name_with_extension = file_name.split("\\").last();

            let param_name = if let Some(file_name_with_extension) = file_name_with_extension {
                file_name_with_extension.split(".").nth(0)
            } else {
                return Err(DekuError::Parse(Cow::from(format!(
                    "Failed to get param name from: {file_name})"
                ))));
            };

            let param_name = if let Some(param_name) = param_name {
                param_name
            } else {
                return Err(DekuError::Parse(Cow::from(format!(
                    "Failed to get param name from: {file_name})"
                ))));
            };

            if bytes_read < data_offset {
                reader.skip_bits((data_offset - bytes_read) * 8)?;
            }
            let mut file_data = vec![0; compressed_size];
            reader.read_bytes(compressed_size, &mut file_data)?;
            param_files.insert(param_name.to_string(), file_data);
        }

        Ok(Self { param_files })
    }
}
impl<'a> DekuWriter<Ctx<'a>> for Params {
    fn to_writer<W: std::io::Write>(
        &self,
        writer: &mut Writer<W>,
        ctx: Ctx<'a>,
    ) -> Result<(), DekuError> {
        let (_, file_headers, _) = ctx;
        for ((_, param_file), file_header) in self.param_files.iter().zip(file_headers) {
            let data_offset = file_header.data_offset as usize;
            param_file.to_writer(writer, ())?;
            if param_file.len() < data_offset {
                let padding = vec![0; data_offset - param_file.len()];
                padding.to_writer(writer, ())?;
            }
        }
        Ok(())
    }
}
