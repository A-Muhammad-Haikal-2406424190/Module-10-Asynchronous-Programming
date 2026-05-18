use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio::sync::Mutex;
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IncomingMessage {
    message_type: MsgTypes,
    data: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OutgoingMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Debug, Serialize)]
struct MessageData {
    from: String,
    message: String,
    time: u128,
}

type SharedUsers = Arc<Mutex<HashMap<SocketAddr, String>>>;

fn build_users_message(usernames: Vec<String>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&OutgoingMessage {
        message_type: MsgTypes::Users,
        data_array: Some(usernames),
        data: None,
    })
}

fn build_chat_message(from: String, message: String) -> Result<String, serde_json::Error> {
    let message_data = MessageData {
        from,
        message,
        time: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis()),
    };

    serde_json::to_string(&OutgoingMessage {
        message_type: MsgTypes::Message,
        data_array: None,
        data: Some(serde_json::to_string(&message_data)?),
    })
}

async fn broadcast_users(
    users: &SharedUsers,
    bcast_tx: &Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let usernames = {
        let locked = users.lock().await;
        let mut usernames: Vec<String> = locked.values().cloned().collect();
        usernames.sort();
        usernames
    };

    bcast_tx.send(build_users_message(usernames)?)?;
    Ok(())
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: SharedUsers,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From client {addr:?}: {text}");

                            match serde_json::from_str::<IncomingMessage>(text) {
                                Ok(parsed) => {
                                    match parsed.message_type {
                                        MsgTypes::Register => {
                                            if let Some(username) = parsed.data {
                                                users.lock().await.insert(addr, username);
                                                broadcast_users(&users, &bcast_tx).await?;
                                            }
                                        }
                                        MsgTypes::Message => {
                                            if let Some(message) = parsed.data {
                                                let sender_name = users
                                                    .lock()
                                                    .await
                                                    .get(&addr)
                                                    .cloned()
                                                    .unwrap_or_else(|| addr.to_string());

                                                bcast_tx.send(build_chat_message(sender_name, message)?)?;
                                            }
                                        }
                                        MsgTypes::Users => {
                                            broadcast_users(&users, &bcast_tx).await?;
                                        }
                                    }
                                }
                                Err(_) => {
                                    // Backward compatibility for the tutorial-2 terminal client:
                                    // if text is not JSON, treat it as a plain chat message.
                                    let sender_name = users
                                        .lock()
                                        .await
                                        .get(&addr)
                                        .cloned()
                                        .unwrap_or_else(|| addr.to_string());
                                    bcast_tx.send(build_chat_message(sender_name, text.to_string())?)?;
                                }
                            }
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => {
                        let removed = users.lock().await.remove(&addr);
                        if removed.is_some() {
                            broadcast_users(&users, &bcast_tx).await?;
                        }
                        return Ok(());
                    },
                }
            }
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let users: SharedUsers = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        let users = users.clone();
        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;
            handle_connection(addr, ws_stream, bcast_tx, users).await
        });
    }
}
