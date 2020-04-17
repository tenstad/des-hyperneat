echo "Profiling with $1 iterations"
export ITERATIONS="$1"
time cargo flamegraph --bin=des-hyperneat
python3 flame_highlight.py
rm perf.data*
/opt/google/chrome/chrome flamegraph.svg
