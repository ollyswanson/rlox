# Crafting Interpreters in Rust

## Part 1 - Treewalk Interpreter

### Performance

TODO: Proper write benchmarks and write up.

Naive benchmarking shows the Rust implementation of the tree-walk interpreter
to be performant relative to the Java implementation, but pathological recursion
such as with recursive fibonacci is slower than the Java implementation due to
lots of small allocations for the `2^n` function calls.
