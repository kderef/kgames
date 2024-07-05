fn main() {
    if matches!(std::env::var("CARGO_PROFILE"), Ok(v) if v == "release") {
        static_vcruntime::metabuild();
    }
}