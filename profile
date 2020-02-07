echo "Profiling with $1 iterations"
export ITERATIONS="$1"
cargo flamegraph
python3 flame_highlight.py
/opt/google/chrome/chrome flamegraph.svg

