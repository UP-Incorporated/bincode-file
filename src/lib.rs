use bincode::config::standard;
use bincode::de::Decode;
use bincode::decode_from_slice;
use std::error::Error;
use std::path::Path;

/// Reads and decodes a bincode-serialized file into type `T`.
pub fn read<T>(path: &Path) -> Result<T, Box<dyn Error>>
where
    T: Decode<()>, // T must implement bincode::Decode with default config
{
    // Read file into memory
    let data = std::fs::read(path)?;

    // Decode using bincode; propagate errors
    let (value, _bytes_read) = decode_from_slice::<T, _>(&data, standard())?;
    Ok(value)
}
pub fn write<T: bincode::Encode>(path: &Path, value: &T) -> Result<(), Box<dyn Error>>
{
    let encoded_data = bincode::encode_to_vec(value, standard())?;
    std::fs::write(path, encoded_data)?;
    Ok(())
}
