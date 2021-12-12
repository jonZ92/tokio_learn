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

use std::fmt::Result;

use tokio::net::TcpListener;

use tokio::io::{AsyncReadExt,AsyncWriteExt};

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
async fn main()->Result<(),Box<dyn std::error::Error>>{


    Ok(())
}