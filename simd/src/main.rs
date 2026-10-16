use fearless_simd::{Level, dispatch, prelude::*};
use fearless_simd_macros::simd;
use shared::harness;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let level = Level::new();
    harness::run(
        "explicit SIMD",
        |haystack, needle| dispatch!(level, simd => count_bytes_simd(simd, haystack, needle)),
    )
}

#[simd]
fn count_bytes_simd<S: Simd>(simd: S, haystack: &[u8], needle: u8) -> usize {
    let needle_vector = S::u8s::splat(simd, needle);

    let haystack_chunks = haystack.chunks_exact(S::u8s::LEN);
    let remainder = haystack_chunks.remainder();
    let mut count: usize = 0;
    for chunk in haystack_chunks {
        let haystack_vector = S::u8s::from_slice(simd, chunk);
        let comparison = needle_vector.simd_eq(haystack_vector);
        count += comparison.to_bitmask().count_ones() as usize;
    }

    let remainder_bytes = remainder.iter().filter(|&&byte| byte == needle).count();
    count += remainder_bytes;

    count
}
