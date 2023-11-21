use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
pub fn read_from_socket(stream: &mut TcpStream) -> io::Result<Option<Vec<u8>>> {
    let mut data = Vec::new();
    let mut buffer = [0; 4096];

    loop {
        match stream.read(&mut buffer) {
            Ok(4096) => {
                data.extend_from_slice(&buffer[..4096]);
            }
            Ok(bytes_read) => {
                data.extend_from_slice(&buffer[..bytes_read]);
                break;
            }
            Err(e) => return Err(e),
        }
    }

    if data.is_empty() {
        Ok(None)
    } else {
        Ok(Some(data))
    }
}

pub fn write_to_socket(stream: &mut TcpStream, data: Vec<u8>) -> io::Result<()> {
    let mut bytes_written = 0;
    let total_size = data.len();
    //TODO send 4 bytes for size?
    while bytes_written < total_size {
        match stream.write(&data[bytes_written..]) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write to stream",
                ));
            }
            Ok(n) => bytes_written += n,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
