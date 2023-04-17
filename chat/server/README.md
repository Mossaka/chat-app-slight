# A simple chat server using SpiderLightning, slight, Rust and Redis

## How to run?
- Run redis server: `redis-server`
- Make sure redis server is running: `redis-cli ping`
- Install slight. Please see instructions [here](https://github.com/deislabs/spiderlightning#how-to-install-on-macos-and-linux)
- Run slight `REDIS_ADDRESS=redis://127.0.0.1:6379 RUST_LOG=slight=trace slight -c slightfile.toml run -m ./target/wasm32-wasi/release/http-demo.wasm`
