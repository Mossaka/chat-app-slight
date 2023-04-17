build:
	cargo build --target wasm32-wasi --manifest-path ./chat/server/Cargo.toml --release

run:
	RUST_LOG=slight=trace slight -c ./chat/server/slightfile.toml run ./target/wasm32-wasi/release/chat_server.wasm

smoke: build
	cargo run --manifest-path ./chat/server/Cargo.toml --bin simulate

benchmark: build
	cargo run --manifest-path ./chat/server/Cargo.toml --bin benchmark

serve:
	trunk serve ./chat/client/index.html

clean:
	rm ~/dump.rdb
	sudo rm -rf /tmp/?????????????????????????????
