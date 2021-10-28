use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // set up TCP listener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // accept client connection
    let (mut socket, _addr) = listener.accept().await.unwrap();

    let (reader, mut writer) = socket.split();

    let mut reader = BufReader::new(reader);
    let mut line = String::new();
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
}
