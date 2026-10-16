use std::{env, error::Error, fs, hint::black_box, time::Instant};

pub fn run(label: &str, count_bytes: impl Fn(&[u8], u8) -> usize) -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(format!("Usage: {} <text-file> <single-byte-character>", args[0]).into());
    }

    let needle = args[2].as_bytes();
    if needle.len() != 1 {
        return Err("Please pass exactly one byte to count".into());
    }
    let needle = needle[0];
    let text = fs::read_to_string(&args[1])?;
    let bytes = text.as_bytes();

    const WARM_ITER: u32 = 16;
    const BENCH_ITER: u32 = 256;

    for _ in 0..WARM_ITER {
        black_box(count_bytes(black_box(bytes), black_box(needle)));
    }

    let start = Instant::now();
    let mut count = 0;
    for _ in 0..BENCH_ITER {
        count = black_box(count_bytes(black_box(bytes), black_box(needle)));
    }
    let elapsed = start.elapsed();

    println!("Number of occurrences ({label}): {count}");
    println!("Elapsed time ({label}): {elapsed:?}");
    println!(
        "Elapsed time per iteration ({label}): {:?}",
        elapsed / BENCH_ITER
    );
    Ok(())
}
