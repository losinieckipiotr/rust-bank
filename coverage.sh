cargo clean

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage"

cargo build

export LLVM_PROFILE_FILE="rust-bank-%p-%m.profraw"

cargo test

grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

# windows
start target/debug/coverage/index.html

rm *.profraw
