use std::sync::Arc;

use streamfer::main::infer::Cap;
use wasmtime::{component::*, Store};

wasmtime::component::bindgen!("streamfer");

pub struct State {}

impl streamfer::main::infer::Host for State {
    fn get_capabilities(&mut self) -> wasmtime::Result<Vec<Cap>> {
        Ok(vec![Cap::CapCodecH264])
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("WasmtimeError: {0}")]
    WasmtimeError(#[from] wasmtime::Error),
}

pub struct Engine {
    wasmtime_engine: wasmtime::Engine,
}

impl Engine {
    pub fn new() -> Result<Self, Error> {
        let mut config = wasmtime::Config::new();
        config.wasm_component_model(true).parallel_compilation(true);
        config.wasm_threads(true);

        Ok(Self {
            wasmtime_engine: wasmtime::Engine::new(&config)?,
        })
    }

    pub fn load_module(self: Arc<Self>, wasm: &[u8]) -> Result<Module, Error> {
        let component = Component::from_binary(&self.wasmtime_engine, wasm)?;
        let mut store = Store::new(&self.wasmtime_engine, State {});
        let mut linker = Linker::new(&self.wasmtime_engine);

        Streamfer::add_to_linker(&mut linker, |s: &mut State| s)?;

        let (streamfer, _) = Streamfer::instantiate(&mut store, &component, &linker)?;

        Module::new(self.clone(), streamfer, store)
    }
}

pub struct Module {
    engine: Arc<Engine>,
    streamfer: Streamfer,
    store: Store<State>,
}

impl Module {
    fn new(engine: Arc<Engine>, streamfer: Streamfer, store: Store<State>) -> Result<Self, Error> {
        Ok(Module {
            engine,
            streamfer,
            store,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut results = [Val::Bool(false)];
        self.streamfer
            .run
            .call(&mut self.store, &[], &mut results)?;

        println!("{:?}", results);

        Ok(())
    }
}
