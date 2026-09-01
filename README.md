# Partial borrow without locks

A tiny Rust example for a common large-system shape: one global-ish
`Compilation` contains two logically independent graphs. An operation needs to
read `ModuleGraph` while it updates `ChunkGraph`.

```sh
cargo run
```

## The point

Global lifetime does **not** imply shared mutation or a lock. If the operation
really reads one field and writes another, model that fact directly:

```rust
let Compilation { module_graph, chunk_graph } = self;
chunk_graph.module_counts.push(module_graph.modules.len());
```

Rust accepts this because the borrows are disjoint and their safety is still
checked statically. No `Arc`, `Mutex`, `RefCell`, runtime borrow failure,
deadlock risk, or synchronization overhead is needed for this operation.

`Arc` remains useful for genuinely shared ownership, and a lock is useful for
genuinely concurrent mutable access. They should not be the default workaround
for a purely structural borrow problem.

## Why this matters in a God Object

God Objects are often worth decomposing, but they also occur in real compiler,
build, and database systems. While refactoring, a partial-borrow-friendly API
lets code express the actual dependency boundary rather than turning an
otherwise single-threaded mutation into interior mutability.

## Discussion context

This repository is a runnable companion to the discussion at
https://x.com/hardfist_1/status/2094659037971595659.

## License

MIT.
