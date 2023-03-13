use std::{net::TcpListener, io::Write, thread::sleep, time::Duration};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    match listener.accept() {
        Ok((mut stream, addr)) => {
            println!("Connected to peer at {:?}", addr);
            loop {
                let buffer = [10, 23, 45, 24];
                match stream.write(&buffer) {
                    Ok(usize) => {
                        println!("Wrote {:?} bytes to peer", usize);
                    }
                    Err(error) => {
                        println!("Connection with peer lost: {:?}", error);
                        break;
                    }
                }
                sleep(Duration::from_secs(1));
            };
        },
        Err(error) => {
            println!("Failed to connect to peer: {:?}", error);
        }
    }
    Ok(())
}
