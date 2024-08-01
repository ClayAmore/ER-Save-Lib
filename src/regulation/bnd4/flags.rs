#[allow(unused)]
#[repr(u8)]
pub(crate) enum Format {
    None = 0,
    BigEndian = 0b0000_0001,
    IDs = 0b0000_0010,
    Names1 = 0b0000_0100,
    Names2 = 0b0000_1000,
    LongOffsets = 0b0001_0000,
    Compression = 0b0010_0000,
    Flag6 = 0b0100_0000,
    Flag7 = 0b1000_0000,
}

#[allow(unused)]
#[repr(u8)]
pub(crate) enum FileFlags {
    None = 0,
    Compressed = 0b0000_0001,
    Flag1 = 0b0000_0010,
    Flag2 = 0b0000_0100,
    Flag3 = 0b0000_1000,
    Flag4 = 0b0001_0000,
    Flag5 = 0b0010_0000,
    Flag6 = 0b0100_0000,
    Flag7 = 0b1000_0000,
}
