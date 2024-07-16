use core::result::Result;
use deku::ctx::Endian;
use deku::prelude::*;
use deku::{DekuRead, DekuWrite};
use std::fs::File;
use std::path::Path;
use std::{
    fs::{self},
    io::{self, Cursor},
};
use thiserror::Error;

use super::{user_data_10::UserData10, user_data_11::UserData11, user_data_x::UserDataX};

#[derive(Error, Debug)]
pub enum SaveParseError {
    #[error(transparent)]
    DekuError(#[from] DekuError),
    #[error(transparent)]
    IoError(#[from] io::Error),
}

/// A struct representing an Elden Ring save file.
///
/// This struct provides methods to read a save file from a specified file path or from
/// a byte slice, and to check if a byte slice can be parsed as a valid save file.
/// It includes functions for reading, parsing, and validating save files, ensuring that
/// implementations handle the respective file formats correctly.
///
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: Endian, is_ps: bool, size: [usize; 4]"
)]
pub struct Save {
    // Magic
    pub magic: [u8; 4],

    // Header
    #[deku(count = "size[0]")]
    header: Vec<u8>,

    // UserData 0-9
    #[deku(
        reader = "UserDataX::read(
            deku::reader,
            Endian::Little,
            deku::byte_offset,
            size[1], 
            10, 
            is_ps
        )",
        writer = "UserDataX::write(
            deku::writer,
            Endian::Little,
            deku::byte_offset,
            size[1], 
            is_ps,
            &self.user_data_x
        )"
    )]
    pub(crate) user_data_x: Vec<UserDataX>,

    // UserData 10
    #[deku(
        reader = "UserData10::read(
            deku::reader,
            Endian::Little,
            deku::byte_offset, 
            size[2], 
            is_ps
        )",
        writer = "UserData10::write(
            deku::writer,
            Endian::Little,
            deku::byte_offset, 
            size[2], 
            is_ps,
            &self.user_data_10
        )"
    )]
    pub(crate) user_data_10: UserData10,

    // UserData 11
    #[deku(
        reader = "UserData11::read(
            deku::reader,
            Endian::Little,
            deku::byte_offset, 
            size[3], 
            is_ps
        )",
        writer = "UserData11::write(
            deku::writer,
            Endian::Little,
            deku::byte_offset, 
            size[3], 
            is_ps,
            &self.user_data_11
        )"
    )]
    pub(crate) user_data_11: UserData11,
}

impl Save {
    fn read<R: std::io::Read>(
        reader: &mut deku::reader::Reader<R>,
        is_ps: bool,
    ) -> Result<Self, DekuError> {
        let sizes: [usize; 4] = if is_ps {
            [0x6c, 0x280000, 0x60000, 0x240010]
        } else {
            [0x2fc, 0x280010, 0x60010, 0x240020]
        };
        let mut save = Self::from_reader_with_ctx(reader, (Endian::Little, is_ps, sizes))?;
        save.user_data_x[0].update()?;
        Ok(save)
    }

    /// Writes the save file to a byte slice and returns it as a `Vec<u8>`.
    ///
    /// This function converts the `Save` instance into a byte slice, which can then be
    /// saved to a file or used as needed.
    ///
    /// # Errors
    ///
    /// This function will return an `Err` variant if any of the following conditions occur:
    /// - There are issues with formatting the data into a byte slice.
    ///
    /// Possible error types include:
    /// - `deku::DekuError` if the bytes cannot be formatted correctly.
    ///
    /// # Examples
    ///
    /// ```
    /// use er_save_lib::Save;
    ///
    /// fn main() {
    ///     let save = Save::from_path("./test/ER0000.sl2").expect("Failed to read save file!");
    ///     let bytes = save.to_slice().expect("Failed to write save file!");
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// This function is safe to call as it only performs data formatting operations.
    pub fn write_to_slice(&self) -> Result<Vec<u8>, DekuError> {
        let is_ps = self.header.len() == 0x6c;
        let sizes: [usize; 4] = if is_ps {
            [0x6c, 0x280000, 0x60000, 0x240010]
        } else {
            [0x2fc, 0x280010, 0x60010, 0x240020]
        };
        let mut buffer = Vec::new();
        {
            let mut temp_writer = Writer::new(Cursor::new(&mut buffer));
            self.to_writer(&mut temp_writer, (Endian::Little, is_ps, sizes))?;
        }
        Ok(buffer)
    }

    /// Writes the save file to a specified path.
    ///
    /// This function attempts to write the `Save` instance to a file at the given path.
    ///
    /// # Errors
    ///
    /// This function will return an `Err` variant if any of the following conditions occur:
    /// - The file cannot be created or opened for writing.
    /// - There are issues with writing the data to the file.
    ///
    /// Possible error types include:
    /// - `std::io::Error` if there are issues with file access.
    /// - `deku::DekuError` if the bytes cannot be formatted correctly.
    ///
    /// # Examples
    ///
    /// ```
    /// use er_save_lib::Save;
    ///
    /// fn main() {
    ///     let save = Save::from_path("./test/PS_Save.txt").expect("Failed to read save file!");
    ///     save.write_to_path("./test/New_Save.txt").expect("Failed to write save file!");
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// This function is safe to call as it only performs file writing and data formatting operations.
    pub fn write_to_path(&self, path: &str) -> Result<(), SaveParseError> {
        let path = Path::new(path);
        let file = if !path.exists() {
            File::create(path)?
        } else {
            File::options().write(true).open(path)?
        };
        let is_ps = self.header.len() == 0x6c;
        let sizes: [usize; 4] = if is_ps {
            [0x6c, 0x280000, 0x60000, 0x240010]
        } else {
            [0x2fc, 0x280010, 0x60010, 0x240020]
        };

        let mut writer = Writer::new(file);
        self.to_writer(&mut writer, (Endian::Little, is_ps, sizes))?;

        Ok(())
    }

    /// Reads a save file from the specified path and returns a `Save` instance.
    ///
    /// This function attempts to read the file at the given path, parse its contents
    /// as a `Save` structure, and return it wrapped in a `Result`.
    ///
    /// # Errors
    ///
    /// This function will return an `Err` variant if any of the following conditions occur:
    /// - The file does not exist at the specified path.
    /// - There are insufficient permissions to read the file.
    /// - The file is not valid or cannot be parsed as a `Save` structure.
    ///
    /// Possible error types include:
    /// - `std::io::Error` if there are issues with file access.
    /// - `deku::DekuError` if the bytes cannot be parsed correctly.
    ///
    /// # Examples
    ///
    /// ```
    /// use er_save_lib::Save;
    ///
    /// fn main() {
    ///     let save = Save::from_path("./test/PS_Save.txt").expect("Failed to read save file!");
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not explicitly panic. However, if it is called in a context
    /// where the `Result` is not handled properly, it could cause a panic.
    ///
    /// # Safety
    ///
    /// This function is safe to call as it only performs file reading and parsing operations.
    pub fn from_path(path: &str) -> Result<Self, SaveParseError> {
        let bytes = fs::read(path)?;
        Self::from_slice(&bytes)
    }

    /// Parses a byte slice and returns a `Save` instance.
    ///
    /// This function attempts to parse the provided byte slice as a `Save` structure
    /// and return it wrapped in a `Result`.
    ///
    /// # Errors
    /// - `std::io::Error` if there are issues with file access.
    /// - `deku::DekuError` if the bytes cannot be parsed correctly.
    ///
    /// This function will return an `Err` variant if the byte slice cannot be parsed
    /// as a valid `Save` structure.
    ///
    /// Possible error types include:
    /// - `deku::DekuError` if the bytes cannot be parsed correctly.
    ///
    /// # Examples
    ///
    /// ```
    /// use er_save_lib::Save;
    ///
    /// fn main() {
    ///     let bytes = std::fs::read("./test/PS_Save.txt").unwrap();
    ///     let save = Save::from_slice(&bytes).expect("Failed to read save file!");
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not explicitly panic. However, if it is called in a context
    /// where the `Result` is not handled properly, it could cause a panic.
    ///
    /// # Safety
    ///
    /// This function is safe to call as it only performs parsing operations on the provided
    /// byte slice.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, SaveParseError> {
        let is_ps = Self::is_ps(bytes);
        let mut cursor = Cursor::new(bytes);
        let mut reader = Reader::new(&mut cursor);
        let save = Self::read(&mut reader, is_ps)?;
        Ok(save)
    }

    /// Checks if the provided byte slice can be parsed as a valid `Save` structure.
    ///
    /// This function performs a quick validation check on the byte slice to determine
    /// if it can be parsed as a `Save` structure. This can be useful for filtering
    /// or validating data before attempting to fully parse it.
    ///
    /// # Examples
    ///
    /// ```
    /// use er_save_lib::Save;
    ///
    /// fn main() {
    ///     let bytes = std::fs::read("./test/ER0000.sl2").unwrap();
    ///     assert!(Save::is(&bytes));
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// This function is safe to call as it only performs validation operations on the provided
    /// byte slice.
    pub fn is(bytes: &[u8]) -> bool {
        Self::is_pc(bytes) || Self::is_ps(bytes)
    }

    /// Checks if the provided byte slice indicates a PlayStation save file.
    ///
    /// This function performs a quick validation check on the byte slice to determine
    /// if it belongs to a PlayStation save file format. This can be useful for filtering
    /// or validating data before attempting to fully parse it.
    ///
    /// # Examples
    ///
    /// ```
    /// use er_save_lib::Save;
    ///
    /// fn main() {
    ///     let bytes = std::fs::read("./test/PS_Save.txt").unwrap();
    ///     assert!(Save::is_ps(&bytes));
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// This function is safe to call as it only performs validation operations on the provided
    /// byte slice.
    pub fn is_ps(bytes: &[u8]) -> bool {
        let file_size = bytes.len();

        let save_file_size = 0x1BA0080;
        if file_size < save_file_size {
            return false;
        }

        let user_data_11_offset = 0x1960070;
        let user_data_11_size = 0x240010;
        let user_data_11_end = user_data_11_offset + user_data_11_size;
        let user_data_11 = &bytes[user_data_11_offset..user_data_11_end];
        let digest = md5::compute(user_data_11);

        return digest
            == md5::Digest([
                0x2E, 0x88, 0x1A, 0x15, 0xAC, 0x05, 0x88, 0x8D, 0xF2, 0xC2, 0x6A, 0xEC, 0xC2, 0x90,
                0x89, 0x23,
            ]);
    }

    /// Checks if the provided byte slice indicates a PC save file.
    ///
    /// This function performs a quick validation check on the byte slice to determine
    /// if it belongs to a PC save file format. This can be useful for filtering
    /// or validating data before attempting to fully parse it.
    ///
    /// # Examples
    ///
    /// ```
    /// use er_save_lib::Save;
    ///
    /// fn main() {
    ///     let bytes = std::fs::read("./test/ER0000.sl2").unwrap();
    ///     assert!(Save::is_pc(&bytes));
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// This function is safe to call as it only performs validation operations on the provided
    /// byte slice.
    pub fn is_pc(bytes: &[u8]) -> bool {
        let file_size = bytes.len();
        if file_size < 4 {
            return false;
        }
        &bytes[0..4] == b"BND4"
    }
}
