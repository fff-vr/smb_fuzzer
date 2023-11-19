use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

fn connect_to_server(addr: &str) -> io::Result<()> {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Client connected to {}", addr);

            let mut buffer = vec![0; 8 * 0x10000]; // 버퍼 크기를 512KB로 설정

            loop {
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            // 서버가 연결을 종료했을 경우
                            println!("Server closed the connection");
                            break;
                        }

                        // 읽은 데이터 처리 (예: 화면에 출력)
                        println!("Received {} bytes from server", bytes_read);
                    }
                    Err(e) => {
                        eprintln!("Failed to read from server: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}. Retrying in 10 seconds...", e);
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
                let message = b"ABCD1234"; // 서버로 보낼 메시지

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
            eprintln!("Failed to connect to server: {}. Retrying in 10 seconds...", e);
            thread::sleep(Duration::from_secs(10));
            connect_and_write_to_server(addr); // 재시도
        }
    }

    Ok(())
}


fn main() -> io::Result<()> {
    let ip_address = "127.0.0.1";
    let start_port = 10023;

    for i in 0..12 {
        let port = start_port + i * 3;
        let addr = format!("{}:{}", ip_address, port);

        thread::spawn(move || {
            if let Err(e) = connect_to_server(&addr) {
                eprintln!("An error occurred: {}", e);
            }
        });
    }
    let start_port = 10022;
    for i in 0..12 {
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
