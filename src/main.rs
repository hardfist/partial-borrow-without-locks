//! Two independent graphs can be borrowed at once without `Arc`, `Mutex`, or
//! interior mutability. The important design choice is to expose their
//! independence in the API.

#[derive(Debug, Default)]
struct ModuleGraph {
    modules: Vec<&'static str>,
}

#[derive(Debug, Default)]
struct ChunkGraph {
    module_counts: Vec<usize>,
}

#[derive(Debug, Default)]
struct Compilation {
    module_graph: ModuleGraph,
    chunk_graph: ChunkGraph,
}

impl Compilation {
    /// The API documents the true dependency: read modules, mutate chunks.
    /// Destructuring gives each operation a borrow of only the field it needs.
    fn record_module_count(&mut self) {
        let Self {
            module_graph,
            chunk_graph,
        } = self;

        chunk_graph.module_counts.push(module_graph.modules.len());
    }
}

fn main() {
    let mut compilation = Compilation {
        module_graph: ModuleGraph {
            modules: vec!["entry", "runtime"],
        },
        ..Default::default()
    };

    compilation.record_module_count();
    println!("{:#?}", compilation);
}
