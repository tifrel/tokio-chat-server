use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    // set up TCP listener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel::<String>(1024);

    // this loops handles multiple clients
    loop {
        // accept client connection
        let (mut socket, _addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                //
                tokio::select! {
                    // getting a packet
                    bytes_read = reader.read_line(&mut line) => {
                        // empty packet -> disconnect
                        if bytes_read.unwrap() == 0 {
                            break;
                        }
                        // broadcast
                        tx.send(line.clone()).unwrap();
                        // flush buffer
                        line.clear();
                    }
                    result = rx.recv() => {
                        let msg = result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap()
                    }
                }
            }
        });
    }
}
