pub fn to_array<const L: usize>(slice: &[u8]) -> [u8; L] {
    slice.try_into().expect("slice with incorrect length")
}
