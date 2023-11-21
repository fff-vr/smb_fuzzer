mod mutate;
mod network;
use lazy_static::lazy_static;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use debug_print::{debug_print, debug_println, debug_eprint, debug_eprintln};
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

fn recv_coverage_from_agent(agent_socket: &mut TcpStream) -> bool {
    debug_println!("[recv_coverage_from_agent] start");
    match network::read_from_socket(agent_socket) {
        Ok(Some(bytes_read)) => {
            let coverage_vector: Vec<u64> = convert_to_u64_vec(bytes_read);
            debug_println!("[recv_coverage_from_agent] end");
            add_unique_elements_to_global(coverage_vector)
        }
        Ok(None) => {
<<<<<<< HEAD
            debug_eprintln!("Failed to read from server: zero cov");
            false
        }
        Err(e) => {
            debug_eprintln!("Failed to read from server: {}", e);
=======
            panic!("Failed to read from server: zero cov");
            false
        }
        Err(e) => {
            panic!("Failed to read from server: {}", e);
>>>>>>> 2417834c57550fee55038da9434871ccbdee24cc
            false
        }
    }
}
fn send_mutate_data(smb_socket: &mut TcpStream,data : Vec<u8>) -> io::Result<()> {
    debug_println!("[send_mutate_data]");
    let length = data.len();

    let mut message = length.to_le_bytes().to_vec();
    message.extend(data);

    match network::write_to_socket(smb_socket, message) {
        Ok(_) => {
            debug_println!("Message sent to server");
        }
        Err(e) => {
<<<<<<< HEAD
            debug_eprintln!("Failed to write to server");
=======
            panic!("Failed to write to server");
            
>>>>>>> 2417834c57550fee55038da9434871ccbdee24cc
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

    let agent_addr = format!("{}:{}", ip_address, agent_port);

    let mut agent_socket = TcpStream::connect(agent_addr).unwrap();
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on port 8080");

    loop {
        send_command_to_agent(&mut agent_socket);
        if let Ok((mut stream, _)) = listener.accept() {
            debug_println!("accpet client");
            let original_bytes = recv_original_data(&mut stream);
<<<<<<< HEAD
            debug_println!("recv original bytess\n{}",original_bytes.len());
=======
            println!("recv original bytess\n{:?}",original_bytes);
>>>>>>> 2417834c57550fee55038da9434871ccbdee24cc
            send_mutate_data(&mut stream,original_bytes);
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
        thread::sleep(Duration::from_secs(60000));
    }
}
