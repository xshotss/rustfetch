# NOTE: This is not meant to be used by users who are downloading Rustfetch.
# If you want to download Rustfetch, go to download/

echo "Running cargo fmt..."
cargo fmt
echo "Running clippy..."
cargo clippy --fix
echo "Running all unit tests..."
cargo test
echo "Building binary..."
cargo build --release
echo "Copying it to download/ directory..."
cp target/release/rustfetch download