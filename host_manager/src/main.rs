use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

fn connect_to_server(addr: &str) -> io::Result<()> {
    loop {
        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                println!("Client connected to {}", addr);

                // 서버와의 통신 로직을 여기에 추가합니다.

                // 예시: 서버에 메시지를 보내기
                let message = format!("Hello from client on port {}", addr);
                if let Err(e) = stream.write_all(message.as_bytes()) {
                    eprintln!("Failed to send message from client on port {}: {}", addr, e);
                    continue;
                }

                // 예시: 서버로부터 데이터 읽기
                let mut buffer = [0; 8 *0x100000];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let received = String::from_utf8_lossy(&buffer);
                        println!("Client on port {} received: {}", addr, received);
                    }
                    Err(e) => {
                        eprintln!("Failed to read from server for client on port {}: {}", addr, e);
                    }
                }

                // 연결 및 통신이 성공적으로 완료되면 루프를 종료합니다.
                break;
            }
            Err(e) => {
                eprintln!("Failed to connect for client on port {}: {}. Retrying in 10 seconds...", addr, e);
                // 10초 동안 대기 후 재시도
                thread::sleep(Duration::from_secs(10));
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let ip_address = "127.0.0.1";
    let start_port = 10023; // 시작 포트 번호

    for i in 0..12 {
        let port = start_port + i * 3; // 포트 번호 계산
        let addr = format!("{}:{}", ip_address, port);

        thread::spawn(move || {
            if let Err(e) = connect_to_server(&addr) {
                eprintln!("An error occurred: {}", e);
            }
        });
    }

    // 메인 스레드를 무한 루프 상태로 유지합니다.
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

