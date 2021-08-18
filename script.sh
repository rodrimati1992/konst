clear;clear;
set -x -e
cargo test --no-default-features --features "__test cmp"
cargo test --no-default-features --features "__test parsing_no_proc"
cargo test --no-default-features --features "__test parsing"
cargo test --no-default-features --features "__test constant_time_slice"
cargo test --no-default-features --features "__test const_generics"
cargo test --no-default-features --features "__test rust_1_55"
cargo test --no-default-features --features "__test rust_1_56"
cargo test --no-default-features --features "__test nightly_mut_refs"
cargo test --no-default-features --features "__test cmp parsing constant_time_slice nightly_mut_refs"
cargo test --no-default-features \
    --features "alloc __test cmp parsing constant_time_slice nightly_mut_refs"