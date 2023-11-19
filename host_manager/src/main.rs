use std::net::TcpStream;
use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:10023")?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let mut cursor = io::Cursor::new(buffer);

    while let Ok(value) = cursor.read_u64::<LittleEndian>() {
        println!("Received value: 0x{:X}", value);
    }

    Ok(())
}

