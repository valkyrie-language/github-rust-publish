cargo component build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/github.wasm projects/publish-wasm32-wasi/github-wasm32-wasi.wasm
jco transpile projects/publish-wasm32-wasi/github-wasm32-wasi.wasm -o projects/publish-wasm32-wasi/src --name index --no-namespaced-exports --multi-memory --valid-lifting-optimization --optimize
