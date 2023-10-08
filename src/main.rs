use std::{env, io::Error, net::SocketAddr, sync::{Arc, Mutex}, collections::HashMap};

use futures_util::{future, StreamExt, TryStreamExt, pin_mut};
use tokio::{
    net::{TcpListener, TcpStream}, time::{sleep, Duration}
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use tokio_tungstenite::tungstenite::protocol::Message;
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

use std::process::Command;

mod docker;

#[tokio::main]
async fn main() -> Result<(), Error> {
    console_subscriber::init();
    let _ = env_logger::try_init();

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let peer_map = PeerMap::new(Mutex::new(HashMap::new()));

    // server status reader
    let docker_run_loop = async {
        loop {
            sleep(Duration::from_millis(3000)).await;

            // broadcast to all
            let peers = peer_map.lock().unwrap();
            let broadcast_recipients = peers
                .iter()
                .map(|(_, ws_sink)| ws_sink);
            let n_listeners = broadcast_recipients.len();

            if n_listeners > 0 {
                let msg = get_status_message();

                log::trace!("transmit server status ({}) to ({}) subscribers", msg, n_listeners);
                println!("transmit server status ({}) to subscribers", n_listeners);

                for recipient in broadcast_recipients {
                    if let Err(e) = recipient.unbounded_send(msg.clone()) {
                        println!("{}", e);
                    }
                }
            } else {
                println!("no listeners, skip message dispatch");
            }
        }
    };

    // websocket client listeners
    let ws_run_loop = async {
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::task::Builder::new()
                .name(&format!("{} listener", &addr))
                .spawn(accept_connection(peer_map.clone(), stream, addr)).unwrap();
        }
    };

    tokio::join!(docker_run_loop, ws_run_loop);

    Ok(())
}

async fn accept_connection(peer_map: PeerMap, stream: TcpStream, addr: SocketAddr) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();

    let msg = Message::Text(format!("ehlo, {}", addr));
    if let Err(e) = tx.unbounded_send(msg) {
        println!("{}", e);
    }

    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

fn get_status_message() -> Message {
    // docker ps --format json
    let output = Command::new("docker")
        .args(["ps", "--format", "json"])
        .output()
        .expect("Failed to execute command");

    let str = match std::str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let containers = str.lines().map(|row| {
        let deserialized: docker::Container = serde_json::from_str(&row).unwrap();
        deserialized
    }).collect::<Vec<_>>();
    let f_containers: Vec<docker::ContainerFront> = containers.iter().map(|c| {
        c.into()
    }).collect::<Vec<_>>();

    let str = serde_json::to_string(&f_containers).unwrap();
    Message::Text(str)
}
