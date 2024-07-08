use std::env;

fn main() {
    let target = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    if target == "wasm" {
        return;
    }

    static_vcruntime::metabuild();
}