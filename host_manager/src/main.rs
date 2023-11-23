mod mutator;
mod protocol;
mod network;
mod tools;
mod input_queue;
use lazy_static::lazy_static;
use std::io::{self};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use debug_print::{debug_println,debug_eprintln};
use crate::mutator::mutate;

lazy_static! {
    static ref GLOBAL_VEC: Mutex<Vec<u64>> = Mutex::new(Vec::new());
}

fn add_unique_elements_to_global(va: Vec<u64>) -> u32 {
    let mut global_vec = GLOBAL_VEC.lock().unwrap();
    let mut new_count = 0;
    for item in va {
        if !global_vec.contains(&item) {
            global_vec.push(item);
            new_count+=1;
        }
    }
    new_count
}
fn convert_to_u64_vec(data: Vec<u8>) -> Vec<u64> {
    data.chunks(8)
        .map(|chunk| {
            let mut val: u64 = 0;
            for &byte in chunk.iter().rev() {
                // 리틀 엔디안으로 처리
                val = val << 8 | byte as u64;
            }
            val
        })
        .collect()
}
fn send_command_to_agent(agent_socket: &mut TcpStream) -> bool {
    debug_println!("[send_command_to_agent] start");
    let start_execute = b"\x12";
    match network::write_to_socket(agent_socket, start_execute.to_vec()) {
        Ok(_) => (),
        Err(e) => {
            panic!("Failed to send start execute: {}", e);
        }
    }
    debug_println!("[send_command_to_agent] end");
    true
}

fn recv_coverage_from_agent(agent_socket: &mut TcpStream) -> u32 {
    debug_println!("[recv_coverage_from_agent] start");
    match network::read_from_socket(agent_socket) {
        Ok(Some(bytes_read)) => {
            let coverage_vector: Vec<u64> = convert_to_u64_vec(bytes_read);
            debug_println!("[recv_coverage_from_agent] end");
            add_unique_elements_to_global(coverage_vector)
        }
        Ok(None) => {
            debug_eprintln!("Failed to read from server: zero cov");
            0
        }
        Err(_) => {
            panic!("Failed to read from server: zero cov");
        }
    }
}
fn send_mutate_data(smb_socket: &mut TcpStream,data : Vec<u8>) -> io::Result<()> {
    debug_println!("[send_mutate_data]");
    let length :u32 = data.len().try_into().unwrap();

    let mut message = length.to_le_bytes().to_vec();
    message.extend(data);
    tools::hexdump("send to smb server",&message);
    match network::write_to_socket(smb_socket, message) {
        Ok(_) => {
            debug_println!("Message sent to server");
        }
        Err(e) => {
            panic!("Failed to write to server {}",e);
            
        }
    }
    Ok(())
}
fn recv_original_data(smb_socket : &mut TcpStream) -> Vec<u8>{
    debug_println!("[recv_original_data] start");
    match network::read_from_socket(smb_socket) {
        Ok(Some(bytes_read)) => {
           bytes_read 
        }
        Ok(None) => {
            debug_eprintln!("Failed to read from server: zero cov");
            vec![]
        }
        Err(e) => {
            debug_eprintln!("Failed to read from server: {}", e);
            vec![]
        }
    }
}

fn connect_to_server() {
    let ip_address = "127.0.0.1";
    let agent_port = 10023;
    let mut loop_count :u64 = 0;

    let agent_addr = format!("{}:{}", ip_address, agent_port);

    let mut agent_socket = TcpStream::connect(agent_addr).unwrap();
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on port 8080");

    loop {
        loop_count+=1;
        if loop_count%1000 == 0{
            println!("fuzz loop = {}",loop_count);
        }
        send_command_to_agent(&mut agent_socket);
        if let Ok((mut stream, _)) = listener.accept() {
            debug_println!("accpet client");
            
            let mut original_bytes = recv_original_data(&mut stream);
            debug_println!("recv original bytess\n{}",original_bytes.len());
            tools::hexdump("original bytes",&original_bytes);
            mutate::smb1_mutate(&mut original_bytes,10.0);
            tools::hexdump("mutated bytes",&original_bytes);
            send_mutate_data(&mut stream,original_bytes).unwrap();
        } else {
            panic!("Failed to accept a client.");
        }
        let new_cov_count =  recv_coverage_from_agent(&mut agent_socket);
        if new_cov_count!=0{
            println!("get new cov {}",new_cov_count);
        }
    }
}

fn main() -> io::Result<()> {
    //TODO create execute vm thread.  That thread is also responsible for analyze crash log.

    connect_to_server();

    loop {
        thread::sleep(Duration::from_secs(60000));
    }
}
