use std::{net::TcpStream, io::Read};

fn main() -> std::io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:8080")?;

    loop {
        let mut buffer: [u8; 4] = [0; 4];
        match client.read(&mut buffer) {
            Ok(usize) => {
                if usize == 0 {
                    println!("Connection shutdown by peer.");
                    break;
                }
                println!("Read {:?} from peer; {:?} bytes total.", &buffer, usize);
            }
            Err(error) => {
                println!("Failed to read from peer: {:?}", error);
                break;
            }
        }
    }
    Ok(())
}
