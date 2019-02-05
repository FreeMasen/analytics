PID="$(getpid analytics)"
cargo build --release
if [ -n "$PID" ]; then
	kill "$PID"
	echo "killing $PID"
fi
./start.sh
