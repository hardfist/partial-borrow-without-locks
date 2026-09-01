#[derive(Default)]
struct ModuleGraph {
    modules: Vec<&'static str>,
}

#[derive(Default)]
struct ChunkGraph {
    optimized_module_count: usize,
}

#[derive(Default)]
struct AssetStore {
    emitted_assets: Vec<&'static str>,
}

#[derive(Default)]
struct Diagnostics {
    warnings: Vec<&'static str>,
}

#[derive(Default)]
struct ResolverCache {
    resolved_requests: usize,
}

#[derive(Default)]
struct RuntimeRequirements {
    entries: Vec<&'static str>,
}

#[derive(Default)]
struct Compilation {
    module_graph: ModuleGraph,
    chunk_graph: ChunkGraph,
    assets: AssetStore,
    diagnostics: Diagnostics,
    resolver_cache: ResolverCache,
    runtime_requirements: RuntimeRequirements,
}

impl Compilation {
    // These accessors intentionally hide the field paths from callers.
    fn module_graph(&self) -> &ModuleGraph {
        &self.module_graph
    }

    fn chunk_graph_mut(&mut self) -> &mut ChunkGraph {
        &mut self.chunk_graph
    }
}

// This is deliberately a separate operation, as it is in a real optimizer.
// Its actual contract is narrow: read the module graph and mutate the chunk graph.
fn optimize_chunk_graph(module_graph: &ModuleGraph, chunk_graph: &mut ChunkGraph) {
    chunk_graph.optimized_module_count = module_graph.modules.len();
}

fn optimize(compilation: &mut Compilation) {
    // Current Rust rejects this with E0502. It sees `module_graph()` as an
    // immutable borrow of the whole `Compilation`, then `chunk_graph_mut()` as
    // a mutable borrow of the whole `Compilation`.
    //
    // A partial-borrow-aware method signature could state that the first method
    // borrows only `module_graph` and the second only `chunk_graph`; these paths
    // are disjoint, so this call should then be accepted.
    optimize_chunk_graph(compilation.module_graph(), compilation.chunk_graph_mut());
}

fn main() {
    let mut compilation = Compilation {
        module_graph: ModuleGraph {
            modules: vec!["entry", "runtime"],
        },
        assets: AssetStore {
            emitted_assets: vec!["main.js"],
        },
        diagnostics: Diagnostics {
            warnings: vec!["asset size exceeds the recommended limit"],
        },
        resolver_cache: ResolverCache {
            resolved_requests: 42,
        },
        runtime_requirements: RuntimeRequirements {
            entries: vec!["__webpack_require__"],
        },
        ..Default::default()
    };
    optimize(&mut compilation);
}
