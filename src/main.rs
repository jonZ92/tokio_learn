/**********************************************************************************************************
 *                    is                              ii            ii        
 *                    is          iissiissiiss        ii    ii               iissiissiiss
 *                    is          ii        ii        ii  ii        is       ii        ii   
 *              isisisisisiis     ii        ii        iiii          is       ii        ii   
 *                    is          ii        ii        ii  ii        is       ii        ii   
 *                    is    s     ii        ii        ii    iii     is       ii        ii   
 *                    isisisi     iissiissiiss        ii            is       iissiissiiss
 *    
 **********************************************************************************************************/

 //TODO: author:jon

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use tokio_learn::error::{ErrorCode, NError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error >> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
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
                    NError::new(ErrorCode::EmptyData).error_description();
                    break;
                }
                println!("msg {}", msg);
                writer.write_all(msg.as_bytes()).await.unwrap();
                msg.clear();
            }
        });
    }
}
