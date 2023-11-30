mod config;
mod mutator;
mod network;
mod protocol;
mod qemu;
mod tools;
use crate::input_queue::Fragments;
use crate::mutator::input_queue::{self, InputQueue};
use crate::mutator::smb3_mutate;
use crate::qemu::execute::execute_linux_vm;
use debug_print::debug_println;
use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

lazy_static! {
    static ref GLOBAL_VEC: Mutex<Vec<u64>> = Mutex::new(Vec::new());
    static ref FUZZ_COUNTER: Mutex<u64> = Mutex::new(0);
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
            if 0xffffffff80000000 < val {
                val
            } else {
                0
            }
        })
        .collect()
}
fn send_command_to_agent(agent_listener: &mut TcpStream) -> bool {
    debug_println!("[send_command_to_agent] start");
    let start_execute = b"\x12";
    match network::write_to_socket(agent_listener, start_execute.to_vec()) {
        Ok(_) => (),
        Err(e) => {
            panic!("Failed to send start execute: {}", e);
        }
    }
    debug_println!("[send_command_to_agent] end");
    true
}

fn recv_coverage_from_agent(agent_listener: &mut TcpStream) -> u32 {
    debug_println!("[recv_coverage_from_agent] start");
    match network::read_from_socket(agent_listener) {
        Ok(bytes_read) => {
            let coverage_vector: Vec<u64> = convert_to_u64_vec(bytes_read);
            debug_println!("[recv_coverage_from_agent] end");
            add_unique_elements_to_global(coverage_vector)
        }
        Err(_) => 0,
    }
}
fn send_data(smb_socket: &mut TcpStream, data: Vec<u8>) -> io::Result<()> {
    debug_println!("[send_data]");

    tools::hexdump("send data", &data);
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
fn recv_data(smb_socket: &mut TcpStream) -> Vec<u8> {
    match network::read_from_socket(smb_socket) {
        Ok(bytes_read) => bytes_read,
        Err(_) => {
            vec![]
        }
    }
}

fn accept_or_crash(listener: &TcpListener, wait_second: u64) -> Option<TcpStream> {
    let timeout = Duration::from_secs(wait_second);
    let start_wait = Instant::now();
    loop {
        match listener.accept() {
            Ok((_socket, _addr)) => {
                return Some(_socket);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_secs(1));
                debug_println!("wait for client");
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
async fn fuzz_loop(id: u32) -> io::Result<()> {
    let mut child = execute_linux_vm(id).await;

    let mut current_loop: u64 = 0;
    let agent_address = format!("0.0.0.0:{}", 12345 + id * 2);
    let proxy_address = format!("0.0.0.0:{}", 12346 + id * 2);

    let agent_listener = TcpListener::bind(agent_address).unwrap();
    let proxy_listener = TcpListener::bind(proxy_address).unwrap();
    proxy_listener.set_nonblocking(true).unwrap();
    agent_listener.set_nonblocking(true).unwrap();

    let mut agent_stream = accept_or_crash(&agent_listener, 240)
        .expect("fail to accept agent command channel. TODO restart qemu");
    let mut i_queue = mutator::input_queue::InputQueue::new();
    agent_stream.set_read_timeout(Some(Duration::new(1, 0)))?;
    loop {
        current_loop += 1;
        {
            let mut fuzz_counter = FUZZ_COUNTER.lock().unwrap();
            *fuzz_counter += 1;
        }

        //TODO move to config
        if current_loop % 5000 == 0 {
            if let Err(e) = child.kill().await {
                eprintln!("fail to kill qemu. {}", e);
            }
            println!("restart vm");
            child = execute_linux_vm(id).await;
            agent_stream = accept_or_crash(&agent_listener, 240)
                .expect("fail to accept agent command channel. TODO restart qemu");

            agent_stream.set_read_timeout(Some(Duration::new(1, 0)))?;
            current_loop = 0;
        }

        //TODO Recv one byte from agent. and check crash here
        send_command_to_agent(&mut agent_stream);

        if let Some(mut client_stream) = accept_or_crash(&proxy_listener, 30) {
            debug_println!("accpet client");
            let mut smb_server = TcpStream::connect("127.0.0.1:445").unwrap();
            client_stream.set_read_timeout(Some(Duration::new(1, 0)))?;
            smb_server.set_read_timeout(Some(Duration::new(1, 0)))?;
            let mut packet_count = 0;
            let mut corpus = HashMap::new();
            loop {
                //TODO check socket OK
                debug_println!("start recv ori data");
                let request_bytes = recv_data(&mut client_stream);
                if request_bytes.len() == 0 {
                    debug_println!("client stream is closed");
                    break;
                }
                debug_println!("success recv request_bytes = {}", request_bytes.len());
                send_data(&mut smb_server, request_bytes).unwrap();
                let mut respone_bytes = recv_data(&mut smb_server);

                match rand::thread_rng().gen_range(1..=20) {
                    1 => {
                        let ratio: u32 = rand::thread_rng().gen_range(1..=20);
                        let fragments =
                            smb3_mutate::smb3_mutate_dumb(&mut respone_bytes, ratio as f32);
                        corpus.insert(packet_count, fragments);
                    }

                    2 | 3 | 4 | 5 if i_queue.get_input(packet_count).len() != 0 => {
                        let ratio: u32 = rand::thread_rng().gen_range(1..=20);
                        let fragments = i_queue.get_input(packet_count);
                        let fragments = smb3_mutate::smb3_mutate_coverage(
                            &mut respone_bytes,
                            ratio as f32,
                            fragments,
                        );
                        corpus.insert(packet_count, fragments);
                    }
                    _ => (), //no mutate
                }
                packet_count += 1;
                debug_println!("send to mutated data");
                send_data(&mut client_stream, respone_bytes).unwrap();
            }
            debug_println!("waiting for coverage");
            let new_cov_count = recv_coverage_from_agent(&mut agent_stream);
            debug_println!("recv coverage");
            if new_cov_count != 0 {
                println!("get new cov {}", new_cov_count);
                i_queue.insert_input(corpus);
            }
            smb_server.shutdown(Shutdown::Both).unwrap();
            client_stream.shutdown(Shutdown::Both).unwrap();
        } else {
            println!("accept timeout from agent. it look like crash. t's check vm log");

            if let Err(e) = child.kill().await {
                eprintln!("fail to kill qemu. {}", e);
            }
            let source_path = format!("../workdir/test{}.txt", id);
            let mut target_path = Path::new("../workdir/save/log.txt").to_path_buf();

            let mut counter = 1;
            while target_path.exists() {
                target_path.set_file_name(format!("log{}.txt", counter));
                counter += 1;
            }

            // 파일 이동
            println!("{}", source_path);
            println!("{}", target_path.display());

            fs::rename(source_path, &target_path).unwrap();

            child = execute_linux_vm(id).await;
            agent_stream = accept_or_crash(&agent_listener, 240)
                .expect("fail to accept agent command channel. TODO restart qemu");
            agent_stream.set_read_timeout(Some(Duration::new(1, 0)))?;
            current_loop = 0;

            //TODO analyze vm log
            //TODO save packet
        }

        debug_println!("packet_count = {}", packet_record.len());
    }
}

async fn fuzz() {
    let instance_num = config::get_instance_num();
    for i in 1..instance_num + 1 {
        tokio::spawn(fuzz_loop(i));
    }
    loop {
        {
            let global_vec = GLOBAL_VEC.lock().unwrap();
            tools::save_vec64_to_file("../coverage.txt".to_string(), global_vec.to_vec());
        }
        thread::sleep(Duration::from_secs(60));
    }
}
fn reply(input_file: String) {
    let reply_packet = match tools::read_from_file(&input_file) {
        Ok(data) => data,
        Err(e) => panic!("Failed to read file: {}", e),
    };

    let agent_listener = TcpListener::bind("0.0.0.0:8081").unwrap();
    let smb_listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    smb_listener.set_nonblocking(true).unwrap();
    agent_listener.set_nonblocking(true).unwrap();
    //TODO sequence packet form need;
    let mut agent_stream =
        accept_or_crash(&agent_listener, 60).expect("fail to accecpt agent socket");
    println!("Server listening on port 8080");

    //TODO Recv one byte from agent. and check crash here
    send_command_to_agent(&mut agent_stream);

    if let Some(mut stream) = accept_or_crash(&smb_listener, 60) {
        let _ = recv_data(&mut stream);
        send_data(&mut stream, reply_packet).unwrap();
    }
    let new_cov_count = recv_coverage_from_agent(&mut agent_stream);
    if new_cov_count != 0 {
        println!("get new cov {}", new_cov_count);
    }
}
fn test() {}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    config::initialize_global_config("../config.json".to_string());
    match args.get(1).map(String::as_str) {
        Some("fuzz") => fuzz().await,
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
