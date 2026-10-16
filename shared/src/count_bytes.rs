#[inline(never)]
pub fn count_bytes(haystack: &[u8], needle: u8) -> usize {
    let mut count = 0;
    for byte in haystack {
        if *byte == needle {
            count += 1;
        }
    }
    count
}
