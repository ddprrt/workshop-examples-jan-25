use chat_loop::chat_loop;
use std::{io::Result, net::SocketAddr};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

async fn _echo(mut socket: TcpStream) -> Result<()> {
    let mut buf = String::new();
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    loop {
        let x = reader.read_line(&mut buf).await?;
        if x == 0 {
            break;
        }
        writer
            .write_all(format!("Echo > {}", buf).as_bytes())
            .await?;
        writer.flush().await?;
        buf.clear();
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum Msg {
    Message(String),
    Disconnect(SocketAddr),
}

async fn _chat(
    socket: TcpStream,
    addr: SocketAddr,
    tx: Sender<(SocketAddr, Msg)>,
    mut rx: Receiver<(SocketAddr, Msg)>,
) -> Result<()> {
    println!("The job version!");
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    // Reading Task: Client -> Task -> Broadcast
    tokio::spawn(async move {
        let mut buf = String::new();
        loop {
            let x = reader.read_line(&mut buf).await;
            if x.is_err() || x.unwrap() == 0 || buf.trim() == "exit" {
                tx.send((addr.clone(), Msg::Disconnect(addr))).unwrap();
                break;
            }
            tx.send((addr.clone(), Msg::Message(buf.to_owned())))
                .unwrap();
            buf.clear();
        }
    });

    // Writing Task: Broadcast -> Client

    tokio::spawn(async move {
        loop {
            if let Ok((other_addr, msg)) = rx.recv().await {
                match msg {
                    Msg::Message(msg) => {
                        if other_addr != addr {
                            writer
                                .write_all(format!("{} > {}", other_addr, msg).as_bytes())
                                .await
                                .unwrap();
                            writer.flush().await.unwrap();
                        }
                    }
                    Msg::Disconnect(other_addr) => {
                        if other_addr == addr {
                            break;
                        }
                    }
                }
            }
        }
        println!("Disconnect {addr}");
    });
    Ok(())
}

async fn _select_chat(
    mut socket: TcpStream,
    addr: SocketAddr,
    tx: Sender<(SocketAddr, String)>,
    mut rx: Receiver<(SocketAddr, String)>,
) -> Result<()> {
    println!("The select version!");
    let mut buf = String::new();
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    loop {
        tokio::select! {
            x = reader.read_line(&mut buf) => {
                if x.is_err() || x.unwrap() == 0 || buf.trim() == "exit" {
                    break;
                }
                tx.send((addr.clone(), buf.to_owned())).unwrap();
                buf.clear();
            },
            msg = rx.recv() => {
                if let Ok((other_addr, msg)) = msg {
                    if other_addr != addr {
                        writer
                            .write_all(format!("{} > {}", other_addr, msg).as_bytes())
                            .await?;
                        writer.flush().await?;
                    }
                }
            }
        }
    }
    Ok(())
}

async fn loop_chat(
    socket: TcpStream,
    _addr: SocketAddr,
    tx: Sender<String>,
    mut rx: Receiver<String>,
) -> Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    chat_loop(&mut writer, &mut reader, &tx, &mut rx).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:8001").await?;
    let (tx, _) = broadcast::channel(100);
    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let rx = tx.subscribe();
        println!("Listening to {}", addr);
        tokio::spawn(loop_chat(socket, addr, tx, rx));
    }
}
