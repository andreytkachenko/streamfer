use std::sync::Arc;

use streamfer_engine::Engine;

fn main() {
    let engine = Arc::new(Engine::new().unwrap());
    let wasm = std::fs::read("./example-module/example-component.wasm").unwrap();
    let mut module = engine.load_module(&wasm).unwrap();
    module.run().unwrap();
}
