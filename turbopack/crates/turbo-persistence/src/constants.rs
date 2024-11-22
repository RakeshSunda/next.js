/// Values larger than this become blob files
pub const MAX_MEDIUM_VALUE_SIZE: usize = 10 * 1024 * 1024;

/// Values larger than this become separate value blocks
pub const MAX_SMALL_VALUE_SIZE: usize = 1024 * 1024;

// This value is chosen so the AQMF fits into 16bit length
/// Maximum number of entries per SST file
pub const MAX_ENTRIES_PER_FILE: usize = 30 * 1024;

/// Finish file when total amount of data exceeds this
pub const DATA_THRESHOLD_PER_FILE: usize = 64 * 1024 * 1024;

/// Maximum RAM bytes for AQMF cache
pub const AQMF_CACHE_SIZE: u64 = 10 * 1024 * 1024;
pub const AQMF_AVG_SIZE: usize = 37399;

/// Maximum RAM bytes for key block cache
pub const KEY_BLOCK_CACHE_SIZE: u64 = 100 * 1024 * 1024;
pub const KEY_BLOCK_AVG_SIZE: usize = 4 * 1024 * 1024;

/// Maximum RAM bytes for value block cache
pub const VALUE_BLOCK_CACHE_SIZE: u64 = 100 * 1024 * 1024;
pub const VALUE_BLOCK_AVG_SIZE: usize = 8 * 1024 * 1024;
