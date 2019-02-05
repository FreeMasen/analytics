echo "Starting analytics"
RUST_LOG=trace ./target/release/analytics 2> ~/logs/analytics.log &
