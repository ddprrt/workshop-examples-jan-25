use tokio::sync::broadcast::{Receiver, Sender};
use traits::{ChatSink, ChatStream};

pub mod traits;

pub async fn chat_loop<Sink, Stream>(
    sink: &mut Sink,
    stream: &mut Stream,
    tx: &Sender<Stream::Item>,
    rx: &mut Receiver<Sink::Item>,
) where
    Sink: ChatSink,
    Stream: ChatStream,
{
    loop {
        tokio::select! {
            msg = stream.recv_msg() => {
                if let Ok(result) = msg {
                    let result = tx.send(result);
                    if result.is_err() {
                        break;
                    }
                }
            }
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    let result = sink.send_msg(msg).await;
                    if result.is_err() {
                        break;
                    }
                }
            }
        }
    }
}
