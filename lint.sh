cargo +nightly clippy --fix --allow-dirty --all-features --all-targets
cargo +nightly fmt
cargo sort --workspace -o package,lib,bin,features,dependencies,dev-dependencies,lints,workspace
