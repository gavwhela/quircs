#[derive(Debug)]
pub enum DecodeError {
    InvalidGridSize,
    InvalidVersion,
    DataEcc,
    FormatEcc,
    UnknownDataType,
    DataOverflow,
    DataUnderflow,
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let x = match self {
            DecodeError::InvalidGridSize => "Invalid grid size",
            DecodeError::InvalidVersion => "Invalid version",
            DecodeError::DataEcc => "Format data ECC failure",
            DecodeError::FormatEcc => "ECC failure",
            DecodeError::UnknownDataType => "Unknown data type",
            DecodeError::DataOverflow => "Data overflow",
            DecodeError::DataUnderflow => "Data underflow",
        };
        f.write_str(x)
    }
}

#[derive(Debug)]
pub enum ExtractError {
    OutOfBounds,
}

impl core::fmt::Display for ExtractError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("OutOfBounds")
    }
}
