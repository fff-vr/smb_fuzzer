mod input_queue;
mod mutator;
mod network;
mod protocol;
mod qemu;
mod tools;
use crate::mutator::smb1_mutate;
use crate::qemu::execute::execute_linux_vm;
use std::net::Shutdown;
use crate::protocol::smb3::{self, parse_smb2_header};
use debug_print::{debug_eprintln, debug_println};
use lazy_static::lazy_static;
use std::env;
use std::io::{self, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
lazy_static! {
    static ref GLOBAL_VEC: Mutex<Vec<u64>> = Mutex::new(Vec::new());
}

fn add_unique_elements_to_global(va: Vec<u64>) -> u32 {
    let mut global_vec = GLOBAL_VEC.lock().unwrap();
    let mut new_count = 0;
    for item in va {
        if !global_vec.contains(&item) {
            global_vec.push(item);
            new_count += 1;
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
        Ok(bytes_read) => {
            let coverage_vector: Vec<u64> = convert_to_u64_vec(bytes_read);
            debug_println!("[recv_coverage_from_agent] end");
            add_unique_elements_to_global(coverage_vector)
        }
        Err(_) => {
            panic!("Failed to read from server: zero cov");
        }
    }
}
fn send_mutate_data(smb_socket: &mut TcpStream, data: Vec<u8>) -> io::Result<()> {
    debug_println!("[send_mutate_data]");

    tools::hexdump("send to smb server", &data);
    match network::write_to_socket(smb_socket, data) {
        Ok(_) => {
            debug_println!("Message sent to server");
        }
        Err(e) => {
            panic!("Failed to write to server {}", e);
        }
    }
    Ok(())
}
fn recv_original_data(smb_socket: &mut TcpStream) -> Vec<u8> {
    match network::read_from_socket(smb_socket) {
        Ok(bytes_read) => bytes_read,
        Err(e) => {
            println!("Failed to read from server: {}", e);
            vec![]
        }
    }
}

fn accept_or_crash(listener: &TcpListener) -> Option<TcpStream> {
    let timeout = Duration::from_secs(1000000);
    let start_wait = Instant::now();
    loop {
        match listener.accept() {
            Ok((_socket, _addr)) => {
                return Some(_socket);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                if start_wait.elapsed() >= timeout {
                    return None;
                }
            }
            Err(_) => {
                panic!("unknown error");
            }
        }
    }
}
async fn fuzz_loop() ->io::Result<()>{
    let ip_address = "127.0.0.1";
    let agent_port = 10023;
    let mut loop_count: u64 = 0;

    let agent_addr = format!("{}:{}", ip_address, agent_port);

    let mut agent_socket = TcpStream::connect(agent_addr).unwrap();
    let listener = TcpListener::bind("0.0.0.0:12345").unwrap();
    println!("bind good");
    //let mut i_queue = input_queue::InputQueue::new();
    let mut last_packet = vec![];
    listener.set_nonblocking(true).unwrap();
    println!("Server listening on port 12345");

    loop {
        loop_count += 1;
        if loop_count % 100 == 0 {
            println!("fuzz loop = {}", loop_count);
            //i_queue.print_corpus_count();
        }
        //TODO Recv one byte from agent. and check crash here
        send_command_to_agent(&mut agent_socket);
        //How to detect end?
        if let Some(mut client_stream) = accept_or_crash(&listener) {
            debug_println!("accpet client");
            let mut packet_count = 0;
            let mut smb_server = TcpStream::connect("127.0.0.1:445").unwrap();

            loop {
                //TODO check socket OK
                println!("start recv ori data");
                let request_bytes = recv_original_data(&mut client_stream);
                if request_bytes.len() == 0 {
                    println!("fail to recv ori data");
                    break;
                }
                send_mutate_data(&mut smb_server, request_bytes).unwrap();
                let respone_bytes = recv_original_data(&mut smb_server);
                println!("packet_count = {}", packet_count);
                packet_count += 1;
                //disable queue mode
                let mut corpus = respone_bytes;
                //TODO scheduling fuzz opcode
                /*
                match smb3::parse_command_from_packet(&corpus) {
                    smb3::Smb2Command::SessionSetup => {
                        println!("asdasdasdasd");
                        //smb1_mutate::smb1_mutate(&mut corpus, 10.0);
                    }
                    _ => (),
                }
                */
                last_packet = corpus.clone();
                println!("send to mutated data");
                send_mutate_data(&mut client_stream, corpus).unwrap();
            }

            smb_server.shutdown(Shutdown::Both).unwrap();
            client_stream.shutdown(Shutdown::Both).unwrap();
        } else {
            println!("accept timeout from agent. it look like crash. Let's check vm log");
            //TODO analyze vm log
            let file_path = "last_packet.bin";
            match tools::write_to_file(file_path, &last_packet) {
                Ok(()) => println!("File written successfully"),
                Err(e) => eprintln!("Failed to write file: {}", e),
            }
            panic!("crash occur this is tmp crash. need to proceed fuzzing");
        }
        
        debug_println!("waiting for coverage");
        let new_cov_count = recv_coverage_from_agent(&mut agent_socket);
        debug_println!("recv coverage");
        if new_cov_count != 0 {
            println!("get new cov {}", new_cov_count);
            //i_queue.insert_input(last_packet.clone());
        }
    }
}

fn fuzz() {
    //    let (senders, wait_handles) = execute_linux_vm();
    tokio::spawn(fuzz_loop());
    loop {
        thread::sleep(Duration::from_secs(60000));
    }
}
fn reply(input_file: String) {
    let reply_packet = match tools::read_from_file(&input_file) {
        Ok(data) => data,
        Err(e) => panic!("Failed to read file: {}", e),
    };
    let ip_address = "127.0.0.1";
    let agent_port = 10023;
    let mut loop_count: u64 = 0;

    let agent_addr = format!("{}:{}", ip_address, agent_port);

    let mut agent_socket = TcpStream::connect(agent_addr).unwrap();
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    //TODO sequence packet form need;
    listener.set_nonblocking(true).unwrap();
    println!("Server listening on port 8080");

    //TODO Recv one byte from agent. and check crash here
    send_command_to_agent(&mut agent_socket);

    if let Some(mut stream) = accept_or_crash(&listener) {
        debug_println!("accpet client");

        let mut original_bytes = recv_original_data(&mut stream);
        debug_println!("recv original bytes len =>{}", original_bytes.len());
        debug_println!("send reply packet len => {}", reply_packet.len());
        send_mutate_data(&mut stream, reply_packet).unwrap();
    }
    let new_cov_count = recv_coverage_from_agent(&mut agent_socket);
    if new_cov_count != 0 {
        println!("get new cov {}", new_cov_count);
    }
}
fn test() {}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("fuzz") => fuzz(),
        Some("reply") => {
            if let Some(reply_arg) = args.get(2) {
                reply(reply_arg.to_string());
            } else {
                println!("No additional argument provided for reply");
            }
        }
        Some("test") => test(),
        _ => println!("Unknown command"),
    }
}
