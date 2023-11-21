mod mutate;
mod network;
use lazy_static::lazy_static;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref GLOBAL_VEC: Mutex<Vec<u64>> = Mutex::new(Vec::new());
}

fn add_unique_elements_to_global(va: Vec<u64>) -> bool {
    let mut global_vec = GLOBAL_VEC.lock().unwrap();
    let mut is_new = false;
    for item in va {
        if !global_vec.contains(&item) {
            global_vec.push(item);
            is_new = true;
        }
    }
    is_new
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
    println!("[send_command_to_agent] start");
    let start_execute = b"\x12";
    match network::write_to_socket(agent_socket, start_execute.to_vec()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to send start execute: {}", e);
        }
    }
    println!("[send_command_to_agent] end");
    true
}

fn recv_coverage_from_agent(agent_socket: &mut TcpStream) -> bool {
    println!("[recv_coverage_from_agent] start");
    match network::read_from_socket(agent_socket) {
        Ok(Some(bytes_read)) => {
            let coverage_vector: Vec<u64> = convert_to_u64_vec(bytes_read);
            println!("[recv_coverage_from_agent] end");
            add_unique_elements_to_global(coverage_vector)
        }
        Ok(None) => {
            eprintln!("Failed to read from server: zero cov");
            false
        }
        Err(e) => {
            eprintln!("Failed to read from server: {}", e);
            false
        }
    }
}
fn send_mutate_data(smb_socket: &mut TcpStream) -> io::Result<()> {
    println!("[send_mutate_data]");
    let message = b"\x04\x00\x00\x00ABCD";

    match network::write_to_socket(smb_socket, message.to_vec()) {
        Ok(_) => {
            println!("Message sent to server");
        }
        Err(e) => {
            eprintln!("Failed to write to server");
        }
    }
    Ok(())
}

fn connect_to_server() {
    let ip_address = "127.0.0.1";
    let agent_port = 10023;

    let agent_addr = format!("{}:{}", ip_address, agent_port);

    let mut agent_socket = TcpStream::connect(agent_addr).unwrap();
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on port 8080");

    loop {
        send_command_to_agent(&mut agent_socket);
        if let Ok((mut stream, _)) = listener.accept() {
            println!("accpet client");
            send_mutate_data(&mut stream);
        } else {
            println!("Failed to accept a client.");
        }
        if recv_coverage_from_agent(&mut agent_socket) {
            println!("get new cov");
        }
    }
}

fn main() -> io::Result<()> {
    //TODO create execute vm thread.  That thread is also responsible for analyze crash log.

    connect_to_server();

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
