use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::broadcast;
use tokio::task;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use futures::{StreamExt, SinkExt};

type PeerMap = Arc<Mutex<HashMap<u64, broadcast::Sender<String>>>>;

mod packets;
mod player;
mod server;
mod environment;

#[tokio::main]
async fn main() {
    // Initialize the peer map
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    // Create the game server
    let game_server = Arc::new(Mutex::new(server::Server::new()));

    // Create a broadcast channel for chat and state messages
    let (tx, mut _rx) = broadcast::channel::<String>(100);

    // Spawn the game tick loop
    let server_clone = game_server.clone();
    let tx_clone = tx.clone();
    task::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(100)); // 10 ticks per second
        loop {
            interval.tick().await;

            let state_packet = {
                let mut srv = server_clone.lock().unwrap();
                srv.tick();

                let state = srv.get_state();
                packets::Outgoing::State {
                    tick: srv.tick_count,
                    players: state,
                }
            };

            if let Ok(json) = serde_json::to_string(&state_packet) {
                let _ = tx_clone.send(json);
            }
        }
    });

    // Spawn a task to listen for incoming messages and broadcast them
    let peer_clone = peer_map.clone();
    task::spawn(async move {
        while let Ok(msg) = _rx.recv().await {
            // Broadcast the message to all connected peers
            for (_, sender) in peer_clone.lock().unwrap().iter() {
                let _ = sender.send(msg.clone());
            }
        }
    });

    // WebSocket route handler
    let ws_route = warp::path("chat")
        .and(warp::ws())
        .and(with_peer_map(peer_map.clone()))
        .and(with_broadcast(tx.clone()))
        .and(with_server(game_server.clone()))
        .map(|ws: warp::ws::Ws, peer_map, tx, game_server| {
            ws.on_upgrade(move |socket| user_connected(socket, peer_map, tx, game_server))
        });

    // Serve static files from the "frontend" directory
    let static_files = warp::fs::dir("frontend");

    // Combine the WebSocket route and static files route
    let routes = ws_route.or(static_files);
    // Start the server
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn user_connected(ws: WebSocket, peer_map: PeerMap, gtx: broadcast::Sender<String>, game_server: Arc<Mutex<server::Server>>) {
    // Create a new channel for the connected user
    let (tx, mut _rx) = broadcast::channel::<String>(100);

    // Generate a unique ID for the user
    let user_id = rand::random::<u64>();

    // Save the sender in the peer map
    peer_map.lock().unwrap().insert(user_id, tx.clone());

    // Forward incoming messages from the user to the broadcast channel
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();
    let user_id_clone = user_id;
    println!("USER CONNECTED!");
    tokio::task::spawn(async move {
        let login_name = loop { 
            if let Some(result) = user_ws_rx.next().await {
                let message = match result {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        return;
                    }
                };

                let message = if let Ok(text) = message.to_str() {
                    text.to_owned()
                } else {
                    continue;
                };

                if let Ok(packet) = serde_json::from_str::<packets::Incoming>(&message) {
                    println!("LoginWaiter: It's {:?}!", packet);
                    match packet {
                        packets::Incoming::Login{username} => {
                            game_server.lock().unwrap().login(username.clone());
                            break username;
                        },
                        _ => {}
                    }
                }
            } else {
                return;
            }
        };
        while let Some(result) = user_ws_rx.next().await {
            let message = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
            };

            let message = if let Ok(text) = message.to_str() {
                text.to_owned()
            } else {
                continue;
            };

            println!("GOT MESSAGE {}!", message);
            if let Ok(packet) = serde_json::from_str::<packets::Incoming>(&message) {
                println!("MainLoop: It's {:?}!", packet);
                match packet {
                    packets::Incoming::Chat{message} => {
                        let _ = gtx.send(serde_json::to_string(&packets::Outgoing::Chat{sender: login_name.clone(), message, tick: 0}).unwrap());
                    },
                    packets::Incoming::Walk{x, y} => {
                        game_server.lock().unwrap().player_click_walk(&login_name, x, y);
                    },
                    _ => {}
                }
            }
        }

        // Remove the user from the peer map when the connection is closed
        peer_map.lock().unwrap().remove(&user_id_clone);
        game_server.lock().unwrap().logout(&login_name);
    });

    // Forward broadcast messages to the connected user
    while let Ok(msg) = _rx.recv().await {
        let _ = user_ws_tx.send(Message::text(msg)).await;
    }
}

fn with_peer_map(
    peer_map: PeerMap,
) -> impl Filter<Extract = (PeerMap,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || peer_map.clone())
}

fn with_broadcast(
    gtx: broadcast::Sender<String>,
) -> impl Filter<Extract = (broadcast::Sender<String>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || gtx.clone())
}

fn with_server(
    server: Arc<Mutex<server::Server>>,
) -> impl Filter<Extract = (Arc<Mutex<server::Server>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || server.clone())
}
