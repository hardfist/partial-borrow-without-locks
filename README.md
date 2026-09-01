# Partial borrow without locks

This repository demonstrates a partial-borrow limitation in Rust's ordinary
method signatures.

```sh
cargo check --example current_rust_rejects
```

That command **fails intentionally** with `E0502`.

## The minimal case

`optimize_chunk_graph` is an independent operation:

```rust
fn optimize_chunk_graph(module_graph: &ModuleGraph, chunk_graph: &mut ChunkGraph)
```

It only reads `ModuleGraph` and only mutates `ChunkGraph`. The example's
`Compilation` also carries assets, diagnostics, a resolver cache, and runtime
requirements—typical unrelated state in a compiler/build-system context. The
two graph fields are still disjoint, so the call is memory-safe:

```rust
optimize_module_graph(compilation.module_graph_mut());
optimize_chunk_graph(compilation.module_graph(), compilation.chunk_graph_mut());
```

The first line is a preceding pipeline stage that mutates the module graph; it
finishes before the chunk-graph stage begins. The second line then reads that
optimized module graph while changing the independent chunk graph.

## This is not an immutable-state design

Every field in `Compilation` may be mutated over the lifetime of a compilation.
The relevant guarantee is instead phase-local: stage 1 mutates `ModuleGraph`;
stage 2 mutates `ChunkGraph` while reading the already-optimized
`ModuleGraph`. The other fields may have their own stages.

`Arc` is therefore not a direct solution. It models shared ownership, not the
fact that mutation is confined to a particular field in a particular phase.
`Arc<Mutex<_>>` or interior mutability can force the code to work, but replaces
this statically known, ordered access pattern with synchronization or runtime
borrow checks.

Yet stable Rust rejects it. The problem is not the operation itself; it is that
the public types of the accessor methods are `&self` and `&mut self`. At the
call site, the compiler must conservatively treat each as borrowing the whole
`Compilation`, even though their implementations access different fields.

## The desired partial-borrow behavior

A good partial-borrow mechanism would let a method say which field path it
borrows. In this example, `module_graph()` would expose a shared borrow of only
`module_graph`, while `chunk_graph_mut()` would expose a mutable borrow of only
`chunk_graph`. Since the paths do not overlap, the optimizer call should compile
without `Arc`, `Mutex`, `RefCell`, cloning, or splitting the operation back into
the caller.

Current Rust can accept a one-off workaround if the caller destructures the
fields directly. That is precisely the issue: a reusable, separately defined
optimizer cannot retain the object-oriented accessor boundary while conveying
its narrow borrow footprint.

## Discussion context

This repository is a runnable companion to the discussion at
https://x.com/hardfist_1/status/2094659037971595659.

## License

MIT.
