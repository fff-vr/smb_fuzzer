use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    static ref GLOBAL_VEC: Mutex<Vec<u64>> = Mutex::new(Vec::new());
}

fn read_from_socket(stream: &mut TcpStream) -> io::Result<Option<Vec<u8>>> {
    let mut data = Vec::new();
    let mut buffer = [0; 4096];

    loop {
        match stream.read(&mut buffer) {
            Ok(4096) => {
                data.extend_from_slice(&buffer[..4096]);
                //println!("[read_from_socket] bytes_read = {}",4096);
            },
            Ok(bytes_read) => {
                data.extend_from_slice(&buffer[..bytes_read]);
                //println!("[read_from_socket] bytes_read = {}",bytes_read);
                break;
            }, 
            Err(e) => return Err(e),
        }
    }

    if data.is_empty() {
        Ok(None)
    } else {
        Ok(Some(data))
    }
}

fn add_unique_elements_to_global(va: Vec<u64>) {
    let mut global_vec = GLOBAL_VEC.lock().unwrap();

    for item in va {
        if !global_vec.contains(&item) {
            println!("new cov {:#x}",item);
            global_vec.push(item);
        }
    }
    println!("coverage = {}",global_vec.len());
}
fn convert_to_u64_vec(data: Vec<u8>) -> Vec<u64> {
    data.chunks(8).map(|chunk| {
        let mut val: u64 = 0;
        for &byte in chunk.iter().rev() { // 리틀 엔디안으로 처리
            val = val << 8 | byte as u64;
        }
        val
    }).collect()
}




fn connect_to_server(addr: &str) -> io::Result<()> {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Client connected to {}", addr);


            loop {
                let start_execute = b"\x12";
                match stream.write_all(start_execute) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Failed to send start execute: {}", e);
                    }
                }

                match read_from_socket(&mut stream) {
                    Ok(Some(bytes_read)) => {
                        let coverage_vector: Vec<u64> = convert_to_u64_vec(bytes_read);
                        add_unique_elements_to_global(coverage_vector);
                    }
                    Ok(None)=>{

                        eprintln!("Failed to read from server: zero cov");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to read from server: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!(
                "Failed to connect to agent: {}. Retrying in 10 seconds...",
                e
            );
            thread::sleep(Duration::from_secs(10));
            connect_to_server(addr); // 재시도
        }
    }

    Ok(())
}

fn connect_and_write_to_server(addr: &str) -> io::Result<()> {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Client connected to {}", addr);

            loop {
                let message = b"\x04\x00\x00\x00ABCD"; // 서버로 보낼 메시지

                match stream.write_all(message) {
                    Ok(_) => {
                        println!("Message sent to server on {}", addr);
                    }
                    Err(e) => {
                        eprintln!("Failed to write to server: {}", e);
                        break;
                    }
                }

                // 일정 시간 대기 후 메시지 재전송 (예: 5초)
                thread::sleep(Duration::from_secs(5));
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to smb: {}. Retrying in 10 seconds...", e);
            thread::sleep(Duration::from_secs(10));
            connect_and_write_to_server(addr); // 재시도
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let ip_address = "127.0.0.1";
    let start_port = 10023;
    //TODO create execute vm thread.  That thread is also responsible for analyze crash log.

    for i in 0..1 {
        let port = start_port + i * 3;
        let addr = format!("{}:{}", ip_address, port);

        thread::spawn(move || {
            if let Err(e) = connect_to_server(&addr) {
                eprintln!("An error occurred: {}", e);
            }
        });
    }
    let start_port = 10022;
    for i in 0..1 {
        let port = start_port + i * 3;
        let addr = format!("{}:{}", ip_address, port);

        thread::spawn(move || {
            if let Err(e) = connect_and_write_to_server(&addr) {
                eprintln!("An error occurred: {}", e);
            }
        });
    }

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
