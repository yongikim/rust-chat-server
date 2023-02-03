use std::{env, io::Error};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // コマンドライン引数からアドレスを取得する
    // デフォルトは 127.0.0.1:8080
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // TCPリスナーを作成する
    let socket = TcpListener::bind(&addr).await;
    let listener = socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // 通信を処理する
    while let Ok((socket, _addr)) = listener.accept().await {
        tokio::spawn(async { accept_connection(socket).await });
    }

    Ok(())
}

async fn accept_connection(mut stream: TcpStream) {
    // クライアントのアドレスの表示
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    // ソケットを読み込み部と書き込み部に分割
    let (reader, mut writer) = stream.split();

    // 文字列への読み込み
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();
    loop {
        match buf_reader.read_line(&mut line).await {
            Ok(bytes) => {
                if bytes == 0 {
                    println!("Close connection: {}", addr);
                    break;
                }
            }
            Err(error) => {
                println!("{error}");
                line = "Invalid UTF-8 detected\n".to_string();
            }
        }

        // ソケットへの書き込み（クライアントへの返信）
        writer.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }
}
