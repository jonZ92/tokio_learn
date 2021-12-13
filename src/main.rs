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

use tokio::net::TcpListener;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

/*
#[tokio::main]
async fn main() {
    // tokio::task::spawn_blocking(||{
    //     sleep(Duration::from_millis(10000));
    //     println!("hello ");

    // }).await.unwrap();

    // println!("world");

    // tokio::task::spawn(async {
    //     sleep(Duration::from_millis(5000));
    //     println!("hello");
    // });

    // println!("world");

}
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
   
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            println!("hello world");
            loop {
                let n = match socket.read(&mut buf).await {
                 
                    Ok(n) if n == 0 => {println!("n is = {}",n); return},
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                    
                };
                println!("{:?}",&buf[0..n]);
                let ok="OK".as_bytes();
                if let Err(e) = socket.write_all(&ok).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
