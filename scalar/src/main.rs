use shared::{count_bytes, harness};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    harness::run("scalar", count_bytes)
}
