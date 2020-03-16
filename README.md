# Async Runtimes Comparison

Benchmarks for async_std, tokio and futures runtimes in comparison with synchronous versions.

## How to run

Tests can be run with `cargo bench`.

## Results

From what can be seen in benchmarks we can say:

1. **Synchronous std** version of IO by itself is faster than those of **tokio** and **async_std**.
2. When we have blocking cases of IO and at the same time we can do a computation, it is better to use **async**
3. Tokio, async_std and async_std + futures have roughly the same performance in the presented test cases.
