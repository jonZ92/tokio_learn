
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

use std::thread::sleep;

use tokio::time::Duration;




#[tokio::main]
async fn main() {
    
    tokio::task::spawn_blocking(||{
        sleep(Duration::from_millis(10000));
        println!("hello ");

    }).await.unwrap();

    println!("world");
}
