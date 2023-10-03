wit_bindgen::generate!({
    world: "streamfer",

    exports: {
        world: Streamfer,
    },
});

struct Streamfer;

impl Guest for Streamfer {
    fn run() -> Result<u64, u32> {
        let caps = streamfer::main::infer::get_capabilities();
        println!("{:?}", caps);
        Ok(caps.len() as _)
    }
}
