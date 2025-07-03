/// Converts a reference to any type `T` into a byte slice.
///
/// # Safety
///
/// This function is unsafe because it creates a raw slice from a pointer without
/// bounds checking. The caller must ensure:
///
/// - `data` points to valid memory
/// - `len` does not exceed the actual size of the data pointed to by `data`
/// - The memory remains valid for the lifetime of the returned slice
/// - `len` is not larger than the size of `T` in bytes
///
/// # Arguments
///
/// * `data` - A reference to the data to convert
/// * `len` - The length of the byte slice to create
///
/// # Returns
///
/// A byte slice view of the data
pub unsafe fn to_bytes<T>(data: &T, len: usize) -> &[u8] {
    core::slice::from_raw_parts(data as *const T as *const u8, len)
}
