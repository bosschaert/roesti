# GRCOV_DIR=~/apps/grcov
# echo Directory containing the grcov executable: $GRCOV_DIR

rustup component add llvm-tools-preview
cargo install grcov
cargo clean

echo Run tests producing raw coverage data...
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

echo Execute grcov to collect coverage data and produce HTML report...
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html

# echo Deleting the *.profraw files from the current directory
# rm *.profraw

# open target/coverage/html/index.html

