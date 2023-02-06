use std::{fmt::Display, os::unix::prelude::OsStrExt};

use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{
    io::AsyncBufReadExt,
    io::{AsyncWriteExt, BufReader},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

#[derive(Debug)]
struct ChatCommErr {
    msg: String,
}

impl ChatCommErr {
    fn new() -> Self {
        Self {
            msg: "Error Chatting".to_string(),
        }
    }
}

impl Display for ChatCommErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}", self.msg)
    }
}

impl std::error::Error for ChatCommErr {}

#[async_trait]
trait ChatSink<T> {
    async fn send_msg(&mut self, msg: T) -> Result<(), ChatCommErr>;
}

#[async_trait]
impl<T> ChatSink<T> for SplitSink<WebSocket, T>
where
    Self: SinkExt<T> + Send,
    T: Send,
{
    async fn send_msg(&mut self, msg: T) -> Result<(), ChatCommErr> {
        self.send(msg).await.map_err(|_| ChatCommErr::new())?;
        Ok(())
    }
}

#[async_trait]
impl<T> ChatSink<T> for OwnedWriteHalf
where
    T: Send + 'static + OsStrExt + Sync,
{
    async fn send_msg(&mut self, msg: T) -> Result<(), ChatCommErr> {
        self.write_all(msg.as_bytes())
            .await
            .map_err(|_| ChatCommErr::new())?;
        Ok(())
    }
}

#[async_trait]
trait ChatStream<T> {
    async fn recv_msg(&mut self) -> Result<T, ChatCommErr>;
}

#[async_trait]
impl ChatStream<Message> for SplitStream<WebSocket>
where
    Self: StreamExt + Send,
{
    async fn recv_msg(&mut self) -> Result<Message, ChatCommErr> {
        self.next()
            .await
            .ok_or(ChatCommErr::new())?
            .map_err(|_| ChatCommErr::new())
    }
}

#[async_trait]
impl ChatStream<String> for BufReader<OwnedReadHalf> {
    async fn recv_msg(&mut self) -> Result<String, ChatCommErr> {
        let mut buf = String::new();
        let x = self
            .read_line(&mut buf)
            .await
            .map_err(|_| ChatCommErr::new())?;

        if x == 0 {
            return Err(ChatCommErr::new());
        }

        Ok(buf)
    }
}
