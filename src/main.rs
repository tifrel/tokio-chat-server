use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // set up TCP listener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // this loops handles multiple clients
    loop {
        // accept client connection
        let (mut socket, _addr) = listener.accept().await.unwrap();

        // we need to spawn/move here otherwise on client would block all the others
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            // this loops handles the packets sent by a single client
            loop {
                // read packet
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                // echo back
                writer.write_all(line.as_bytes()).await.unwrap();
                // flush buffer
                line.clear();
            }
        });
    }
}
