/**************************************************************************************
 *
 *
 *
 *
 *
 *
 *
 *
 *
 **************************************************************************************/

// use std::thread::sleep;

// use tokio::time::Duration;




use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use tokio_learn::error::NError;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let listener = TcpListener::bind("cl").await?;

    loop {
        let (mut _stream, _addr) = listener.accept().await?;

        println!("{} connected", _addr);

        tokio::spawn(async move {
            let (reader, mut writer) = _stream.split();

            let mut readers = BufReader::new(reader);

            let mut msg = String::new();

            loop {
                let bytes_read = readers.read_line(&mut msg).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                println!("msg {}", msg);
                writer.write_all(msg.as_bytes()).await.unwrap();
                msg.clear();
            }
        });
    }
}
