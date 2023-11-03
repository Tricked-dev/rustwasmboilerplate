RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort --target wasm32-unknown-unknown --release && \
wasm-opt -Oz --no-validation --disable-gc --all-features --optimize-for-js --vacuum --strip-debug --optimize-casts -o ./library.wasm ./target/wasm32-unknown-unknown/release/wasm_template.wasm
deno run -A test.ts