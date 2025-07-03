pub unsafe fn to_bytes<T>(data: &T, len: usize) -> &[u8] {
    core::slice::from_raw_parts(data as *const T as *const u8, len)
}
