use log::info;
use std::{env, io::Error};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listner we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _addr)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(mut stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let (reader, mut writer) = stream.split();

    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line).await {
            Ok(bytes) => {
                if bytes == 0 {
                    // connection closed
                    break;
                }
                writer.write_all(line.as_bytes()).await.unwrap();
            }
            Err(error) => {
                info!("{error}");
            }
        }
        line.clear();
    }
    // let ws_stream = tokio_tungstenite::accept_async(stream)
    //     .await
    //     .expect("Erro during the websocket handshake occureed");

    // info!("New WebSocket connection: {}", addr);

    // let (write, read) = ws_stream.split();
    // // We should not forward messages other than the text or binary.
    // read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
    //      .forward(write)
    //      .await
    //      .expect("Failed to forward messages")
}
