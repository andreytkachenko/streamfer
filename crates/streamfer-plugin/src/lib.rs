use std::{future::Future, path::Path};

use dlopen2::{
    raw::Library,
    wrapper::{Container, WrapperApi},
};
const COMPONENT_VIDEO_DECODER: &str = "video_decoder_v1";
const COMPONENT_INFERENCE: &str = "inference_v1";
const COMPONENT_PIXEL_OPS: &str = "pixel_ops_v1";
const COMPONENT_TENSOR_OPS: &str = "tensor_ops_v1";

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct MemoryBlock {
    pub address_space: u64,
    pub address: u64,
    pub align: u64,
    pub length: u64,
    pub row_stride: Option<u64>,
    pub row_length: Option<u64>,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct MemoryMap<'a> {
    pub block: MemoryBlock,
    pub slice: &'a [u8],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceMemory {
    pub address_space: u64,
    pub total_size: u64,
    pub free_size: u64,
    pub mappable_to: &'static [u64],
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceInfo {
    pub index: u32,
    pub name: String,
}

#[repr(C)]
#[derive(Debug)]
pub struct Device {
    pub index: u32,
    pub name: String,
    pub memory: DeviceMemory,
}

#[derive(WrapperApi)]
struct PluginApi<'a> {
    #[dlopen2_name = "VERSION"]
    version: &'a &'static str,

    #[dlopen2_name = "VENDOR"]
    vendor: &'a &'static str,

    #[dlopen2_name = "COMPONENTS"]
    components: &'a &'static [&'static str],

    probe: fn() -> Result<Vec<DeviceInfo>, u64>,

    init_device: fn(dev: u32) -> Box<dyn Future<Output = Result<Device, u64>>>,
    // mem_read_async: fn(mem: &MemoryBlock) -> Result<usize, u64>,
    // mem_read_poll: fn(task: usize, cx: &mut Context<'_>) -> Poll<Result<Vec<u8>, u64>>,

    // mem_write_async: fn(
    //     data: &[u8],
    //     device: &DeviceMemory,
    //     row_lenght: Option<u64>,
    //     row_stride: Option<u64>,
    //     cx: &mut Context<'_>,
    // ) -> Poll<Result<MemoryBlock, u64>>,

    // mem_alloc_async: fn(
    //     lenght: u64,
    //     device: &DeviceMemory,
    //     row_lenght: Option<u64>,
    //     row_stride: Option<u64>,
    //     cx: &mut Context<'_>,
    // ) -> Result<MemoryBlock, u64>,

    // mem_clone_async: fn(mem: &MemoryBlock, cx: &mut Context<'_>) -> Poll<Result<MemoryBlock, u64>>,

    // mem_copy_async: fn(
    //     dst: &mut MemoryBlock,
    //     src: &MemoryBlock,
    //     cx: &mut Context<'_>,
    // ) -> Poll<Result<MemoryBlock, u64>>,

    // mem_map_async: fn(mem: MemoryBlock, cx: &mut Context<'_>) -> Poll<Result<MemoryMap<'a>, u64>>,

    // mem_unmap_async: fn(mmap: MemoryMap, cx: &mut Context<'_>) -> Poll<Result<MemoryBlock, u64>>,

    // mem_free: fn(mem: MemoryBlock) -> Result<(), u64>,

    // describe_error: fn(errno: u64) -> Option<&'a str>,
}

#[repr(C)]
struct DecoderConfig {}

#[repr(C)]
struct DeviceCapabilities {}

#[repr(C)]
struct DecodedFrame {
    id: u64,
    device: u64,
    width: u16,
    height: u16,
    channels: u8,
}

// #[derive(WrapperApi)]
// struct DecoderApi<'a> {
//     decoder_enumerate_devices: fn() -> Result<Vec<DeviceInfo>, u64>,
//     decoder_device_capabilities: fn(device: u64) -> Result<DeviceCapabilities, u64>,
//     decoder_create: fn(device: u64, config: DecoderConfig) -> Result<*const (), u64>,
//     decoder_enqueue:
//         fn(decoder: *const (), data: &[u8], timestamp: u64, last_one: bool) -> Result<(), u64>,
//     decoder_dequeue: fn(decoder: *const ()) -> Result<DecodedFrame, u64>,
// }

pub struct DecoderPlugin {}

impl DecoderPlugin {
    pub fn load() {}
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("DlOpenError: {0}")]
    DlOpenError(#[from] dlopen2::Error),
}

pub struct Plugin<'a> {
    lib: Library,
    base: PluginApi<'a>,
    // decoder: Option<DecoderApi<'a>>,
    // inference: Option<InferenceApi<'a>>,
    // image_ops: Option<ImageOpsApi<'a>>,
    // pixel_ops: Option<PixelOpsApi<'a>>,
    // tensor_ops: Option<TensorOpsApi<'a>>,
}

pub struct PluginManager {}

impl PluginManager {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn load<P: AsRef<Path>>(&mut self, prefix: P) -> Result<(), Error> {
        let mut path = prefix.as_ref().to_owned();
        path.push("libstreamfer_plugin_nvdec.so");

        let plugin: Container<PluginApi> = unsafe { Container::load(path) }?;

        let devices = plugin.probe().unwrap();

        println!("{:?} {:?} {:?}", plugin.version, plugin.components, devices);

        println!("{:?}", Box::into_pin(plugin.init_device(0)).await.unwrap());

        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
