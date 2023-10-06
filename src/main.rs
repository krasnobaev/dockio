use std::{env, io::Error, net::SocketAddr};

use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let docker_run_loop = async {
        loop {
            let a = 0;
        }
    };

    let ws_run_loop = async {
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(accept_connection(stream, addr));
        }
    };

    tokio::join!(docker_run_loop, ws_run_loop);

    Ok(())
}

async fn accept_connection(stream: TcpStream, addr: SocketAddr) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages");

    println!("{} disconnected", &addr);
}
