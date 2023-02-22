use std::fmt::Display;

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

// Chat Communication Error
#[derive(Debug)]
pub struct ChatCommErr {
    msg: String,
}

impl ChatCommErr {
    pub fn new() -> Self {
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

// End chat communication error

#[async_trait]
pub trait ChatSink {
    type Item: Clone;
    async fn send_msg(&mut self, msg: Self::Item) -> Result<(), ChatCommErr>;
}

#[async_trait]
pub trait ChatStream {
    type Item: Clone;
    async fn recv_msg(&mut self) -> Result<Self::Item, ChatCommErr>;
}

#[async_trait]
impl<T> ChatSink for SplitSink<WebSocket, T>
where
    Self: SinkExt<T> + Send,
    T: Send + Clone,
{
    type Item = T;
    async fn send_msg(&mut self, msg: Self::Item) -> Result<(), ChatCommErr> {
        self.send(msg).await.map_err(|_| ChatCommErr::new())?;
        Ok(())
    }
}

#[async_trait]
impl ChatSink for OwnedWriteHalf {
    type Item = String;
    async fn send_msg(&mut self, msg: Self::Item) -> Result<(), ChatCommErr> {
        self.write_all(msg.as_bytes())
            .await
            .map_err(|_| ChatCommErr::new())?;
        Ok(())
    }
}

#[async_trait]
impl ChatStream for SplitStream<WebSocket>
where
    Self: StreamExt + Send,
{
    type Item = Message;
    async fn recv_msg(&mut self) -> Result<Self::Item, ChatCommErr> {
        self.next()
            .await
            .ok_or(ChatCommErr::new())?
            .map_err(|_| ChatCommErr::new())
    }
}

#[async_trait]
impl ChatStream for BufReader<OwnedReadHalf> {
    type Item = String;
    async fn recv_msg(&mut self) -> Result<Self::Item, ChatCommErr> {
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
