use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From YewChat client {addr}: {text}");

                            // For YewChat, messages may already be JSON strings.
                            // Therefore, the server forwards the text as-is without adding prefixes.
                            let _ = bcast_tx.send(text.to_string());
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("WebSocket error from {addr}: {e}");
                        return Err(Box::new(e));
                    }
                    None => {
                        println!("Client disconnected: {addr}");
                        return Ok(());
                    }
                }
            }

            msg = bcast_rx.recv() => {
                match msg {
                    Ok(text) => {
                        ws_stream.send(Message::text(text)).await?;
                    }
                    Err(e) => {
                        eprintln!("Broadcast receive error: {e}");
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(32);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Rust WebSocket server for YewChat is listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New YewChat connection from {addr}");

        let bcast_tx = bcast_tx.clone();

        tokio::spawn(async move {
            let result: Result<(), Box<dyn Error + Send + Sync>> = async {
                let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;
                handle_connection(addr, ws_stream, bcast_tx).await
            }
            .await;

            if let Err(e) = result {
                eprintln!("Connection error from {addr}: {e}");
            }
        });
    }
}