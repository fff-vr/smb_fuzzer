use debug_print::debug_print;
use debug_print::debug_println;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::fs::OpenOptions;
#[derive(Serialize, Deserialize)]
struct Config {
    instance_num: u32,
    kernel_path : String,
    vm_path : String
}

lazy_static! {
    static ref GLOBAL_CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}
pub fn hexdump(label: &str, data: &[u8]) {
    debug_println!("{}", label);
    for (i, byte) in data.iter().enumerate() {
        debug_print!("{:02x} ", byte);
        if (i + 1) % 16 == 0 {
            debug_println!();
        }
    }
    debug_println!("\n");
}

pub fn write_to_file(path: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

pub fn read_from_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn read_config(file_path: &str) -> Result<Config,serde_json::Error> {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents)
}
pub fn initialize_global_config(config_path : String) {
    let config = read_config(&config_path).expect("Failed to read config");
    let mut global_config = GLOBAL_CONFIG.lock().unwrap();
    *global_config = Some(config);
}
pub fn get_kernel_path()->String{
    let config = GLOBAL_CONFIG.lock().unwrap();
    if let Some(ref config) = *config {
        config.kernel_path.clone()
    } else {
        panic!("can not get kernel path");
    }
}
pub fn get_vm_path()->String{
    let config = GLOBAL_CONFIG.lock().unwrap();
    if let Some(ref config) = *config {
        config.vm_path.clone()
    } else {
        panic!("can not get vm path");
    }
}
pub fn get_instance_num()->u32{
    let config = GLOBAL_CONFIG.lock().unwrap();
    if let Some(ref config) = *config {
        config.instance_num
    } else {
        panic!("can not get instance_num");
    }
}

pub fn save_vec64_to_file(path : String,vector : Vec<u64>){
    let mut file = OpenOptions::new()
        .write(true)
        .append(true) // append를 true로 설정
        .create(true) // 파일이 존재하지 않을 경우 새로 생성
        .open(path).unwrap();
    for address in vector{
        writeln!(file,"0x{:X}",address).unwrap();
    }
}
