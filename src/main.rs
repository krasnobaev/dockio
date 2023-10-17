use std::{env, io::Error, net::SocketAddr, sync::{Arc, Mutex}, collections::HashMap};

// tokio
use futures_util::{future, StreamExt, TryStreamExt, pin_mut};
use tokio::{
    net::{TcpListener, TcpStream}, time::{sleep, Duration}
};

// websocket
use futures_channel::mpsc::{unbounded, UnboundedSender};
use tokio_tungstenite::tungstenite::protocol::Message;
type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

use std::process::Command;
mod docker;

// http
use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio::fs::File;
use tokio::io::AsyncReadExt; // for read_to_end()

#[tokio::main]
async fn main() -> Result<(), Error> {
    // console_subscriber::init();
    let _ = env_logger::try_init();
    log::info!("======================");
    log::info!("starting dockio server");

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
                log::info!("transmit server status ({}) to subscribers", n_listeners);

                for recipient in broadcast_recipients {
                    if let Err(e) = recipient.unbounded_send(msg.clone()) {
                        log::info!("{}", e);
                    }
                }
            } else {
                log::info!("no listeners, skip message dispatch");
            }
        }
    };

    // websocket client listeners
    let ws_run_loop = async {
        let ws_addr = env::args().nth(1).unwrap_or_else(||
            "0.0.0.0:8081".to_string()
            // "127.0.0.1:8081".to_string()
        );
        let try_socket = TcpListener::bind(&ws_addr).await;
        let listener = try_socket.expect("Failed to bind");
        log::info!("websocket listening on: {}", ws_addr);

        while let Ok((stream, ws_addr)) = listener.accept().await {
            tokio::task::Builder::new()
                .name(&format!("{} listener", &ws_addr))
                .spawn(handle_ws(peer_map.clone(), stream, ws_addr)).unwrap();
        }
    };

    let http_run_loop = async {
        let http_addr = SocketAddr::from(([127, 0, 0, 1], 8080));

        // And a MakeService to handle each connection...
        let make_service = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(handle_http))
        });

        // Then bind and serve...
        let server = Server::bind(&http_addr).serve(make_service);
        log::info!("http listening on: {}", http_addr);

        // And run forever...
        if let Err(e) = server.await {
            log::error!("server error: {}", e);
        }
    };

    tokio::join!(docker_run_loop, ws_run_loop, http_run_loop);

    Ok(())
}

async fn handle_ws(peer_map: PeerMap, stream: TcpStream, addr: SocketAddr) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    log::info!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();

    let file = read_file("dia.drawio.svg").await;
    // let file = read_file("/home/sypwex/prj/ubsl/ccompliance/doc/asset/servers.drawio.svg").await;
    let msg = Message::Binary(file);
    if let Err(e) = tx.unbounded_send(msg) {
        log::error!("{}", e);
    }

    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        match msg.to_text() {
            Ok(msg) => {
                log::info!("Received a command from {}: {}", addr, msg);

                match msg {
                    "Terminate" => {
                        log::warn!("Terminating main process");
                        std::process::exit(0);
                    },
                    _ => {
                        log::warn!("unknown command: {}", msg);
                    },
                }
            },
            Err(_) => todo!(),
        }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    log::info!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

fn get_status_message() -> Message {
    // docker ps --format json
    let output = Command::new("docker")
        .arg("ps")
        .arg("--no-trunc")
        .arg("--format")
        // .arg("json") // Docker 21+
        .arg("{{json .}}") // Docker 20 workaround
        .output()
        .expect("Failed to execute command");

    let str = match std::str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let containers = str.lines().map(|row| {
        // log::debug!("row: {}", row);
        let deserialized: docker::Container = serde_json::from_str(&row).unwrap();
        deserialized
    }).collect::<Vec<_>>();
    let f_containers: Vec<docker::ContainerFront> = containers.iter().map(|c| {
        c.into()
    }).collect::<Vec<_>>();

    let str = serde_json::to_string(&f_containers).unwrap();
    Message::Text(str)
}

async fn handle_http(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    log::debug!("{req:?}");

    let (parts, _body) = req.into_parts();
    let response = match parts.uri.path() {
        "/" | "/index.html" => {
            let file = read_file("index.html").await;
            Response::builder()
                .body(Body::from(file))
                .unwrap()
        },
        "/dia.drawio.svg" => {
            let file = read_file("dia.drawio.svg").await;
            Response::builder()
                .body(Body::from(file))
                .unwrap()
        },
        _ => {
            Response::builder()
              .status(hyper::StatusCode::NOT_FOUND)
              .body(Body::from("not found"))
              .unwrap()
        },
    };

    Ok(response)

}

async fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).await.unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).await.unwrap();

    contents
}
