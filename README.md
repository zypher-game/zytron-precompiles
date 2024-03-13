# zytron-op-precompiles

## Compile

1. compile precompiles
``
cargo build --release
``
2. compile op-geth
``
env GO111MODULE=on go run build/ci.go install ./cmd/geth
``
3. compile all
``
make geth
``
4. get file
``
ls build/bin/geth
ls target/release/libprecompiles.so
``

## Functions

These functions need export as precompiles:

1. Anemoi [execute]() [gas]()

## License

This project is licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html).
