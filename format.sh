cargo +nightly clippy --fix --allow-dirty --target wasm32-unknown-unknown --release
cargo +nightly fix --allow-dirty --target wasm32-unknown-unknown --release
cargo +nightly fmt