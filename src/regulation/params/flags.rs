#[allow(unused)]
#[repr(u8)]
pub(crate) enum FormatFlags {
    None = 0,
    Flag01 = 0b0000_0001,
    IntDataOffset = 0b0000_0010,
    LongDataOffset = 0b0000_0100,
    Flag08 = 0b0000_1000,
    Flag10 = 0b0001_0000,
    Flag20 = 0b0010_0000,
    Flag40 = 0b0100_0000,
    OffsetParamType = 0b1000_0000,
}

#[allow(unused)]
#[repr(u8)]
pub(crate) enum FormatFlags2 {
    None = 0,
    UnicodeRowNames = 0b0000_0001,
    Flag02 = 0b0000_0010,
    Flag04 = 0b0000_0100,
    Flag08 = 0b0000_1000,
    Flag10 = 0b0001_0000,
    Flag20 = 0b0010_0000,
    Flag40 = 0b0100_0000,
    Flag80 = 0b1000_0000,
}
