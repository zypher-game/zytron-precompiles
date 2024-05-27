build:
	cargo build -r
	sudo cp ./target/release/libprecompiles.a /usr/local/lib/