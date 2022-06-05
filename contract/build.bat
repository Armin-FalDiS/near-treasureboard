rd /S /Q res
md res
cargo build --target wasm32-unknown-unknown --release
copy target\wasm32-unknown-unknown\release\*.wasm res