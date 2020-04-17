cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/legion.wasm projects/legion-wasm32-wasi/legion-wasm32-wasi.wasm
jco transpile projects/legion-wasm32-wasi/legion-wasm32-wasi.wasm -o projects/legion-wasm32-wasi/src --name index --no-namespaced-exports --multi-memory --valid-lifting-optimization --optimize

cp target/wasm32-wasip2/release/v.wasm projects/v-wasm32-wasi/v-wasm32-wasi.wasm
jco transpile projects/v-wasm32-wasi/v-wasm32-wasi.wasm -o projects/v-wasm32-wasi/src --name index --no-namespaced-exports --multi-memory --valid-lifting-optimization --optimize