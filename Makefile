build:
	cargo build -r
	sudo cp ./target/release/libprecompiles.so /usr/local/lib/