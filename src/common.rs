pub const MIN_PAGE_NAME_SIZE: usize = 8;
pub const MIN_DIR_NAME_SIZE: usize = 8;
pub const MIN_COL_NAME_SIZE: usize = 8;

pub fn string_to_fixed<const N: usize>(s: &str) -> [u8; N] {
    let mut array = [0u8; N];
    let bytes = s.as_bytes();
    let len = bytes.len().min(N);
    array[..len].copy_from_slice(&bytes[..len]);
    array
}

pub fn fixed_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim_end_matches('\0').to_string()
}