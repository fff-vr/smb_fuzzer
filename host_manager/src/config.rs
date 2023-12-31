use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
#[derive(Serialize, Deserialize)]
struct Config {
    instance_num: u32,
    kernel_path: String,
    vm_path: String,
    ram: String,
    max_loop: u64,
}

lazy_static! {
    static ref GLOBAL_CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

fn read_config(file_path: &str) -> Result<Config, serde_json::Error> {
    let mut file = File::open(file_path).expect("fail to open config");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents)
}
pub fn initialize_global_config(config_path: String) {
    let config = read_config(&config_path).expect("Failed to read config");
    let mut global_config = GLOBAL_CONFIG.lock().unwrap();
    *global_config = Some(config);
}
pub fn get_kernel_path() -> String {
    let config = GLOBAL_CONFIG.lock().unwrap();
    if let Some(ref config) = *config {
        config.kernel_path.clone()
    } else {
        panic!("can not get kernel path");
    }
}
pub fn get_vm_path() -> String {
    let config = GLOBAL_CONFIG.lock().unwrap();
    if let Some(ref config) = *config {
        config.vm_path.clone()
    } else {
        panic!("can not get vm path");
    }
}

pub fn get_instance_num() -> u32 {
    let config = GLOBAL_CONFIG.lock().unwrap();
    if let Some(ref config) = *config {
        config.instance_num
    } else {
        panic!("can not get instance_num");
    }
}
pub fn get_max_loop() -> u64 {
    let config = GLOBAL_CONFIG.lock().unwrap();
    if let Some(ref config) = *config {
        config.max_loop
    } else {
        panic!("can not get instance_num");
    }
}

pub fn get_ram() -> String {
    let config = GLOBAL_CONFIG.lock().unwrap();
    if let Some(ref config) = *config {
        config.ram.clone()
    } else {
        panic!("can not get vm path");
    }
}
