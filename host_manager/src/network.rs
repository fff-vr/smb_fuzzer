use std::io::{self, Read, Write};
use std::net::TcpStream;
pub fn read_from_socket(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    println!("[read_from_socket] start");
    let mut buffer = vec![0; 0x1];
    let bytes_read = stream.read(&mut buffer)?;
    buffer.truncate(bytes_read); // 실제로 읽은 데이터 크기로 버퍼 크기 조정
    println!("[read_from_socket] end");
    Ok(buffer)
}

pub fn write_to_socket(stream: &mut TcpStream, data: Vec<u8>) -> io::Result<()> {
    println!("[write_to_socket] start");
    stream.write_all(&data)?;
    println!("[write_to_socket] end");
    Ok(())
}
