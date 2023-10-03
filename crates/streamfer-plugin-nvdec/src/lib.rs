use std::{future::Future, time::Duration};

#[no_mangle]
pub static VERSION: &str = "0.1.0";

#[no_mangle]
pub static VENDOR: &str = "nvidia";

#[no_mangle]
pub static COMPONENTS: &[&str] = &["decoder"];

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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceMemory {
    pub address_space: u64,
    pub total_size: u64,
    pub free_size: u64,
    pub mappable_to: &'static [u64],
}

#[no_mangle]
pub fn probe() -> Result<Vec<DeviceInfo>, &'static str> {
    Ok(vec![DeviceInfo {
        index: 0,
        name: String::from("CPU"),
    }])
}

#[no_mangle]
pub fn init_device(index: u32) -> Box<dyn Future<Output = Result<Device, u64>>> {
    Box::new(async move {
        // tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(Device {
            index,
            name: String::from("CPU0"),
            memory: DeviceMemory {
                address_space: 0,
                total_size: 100,
                free_size: 100,
                mappable_to: &[],
            },
        })
    })
}
