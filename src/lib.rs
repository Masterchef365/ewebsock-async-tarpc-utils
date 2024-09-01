use serde::{de::DeserializeOwned, Serialize};
use tarpc::Transport;
use bincode::Error as BincodeError;
use std::convert::Infallible;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;

pub fn bincode_stream<Item, SinkItem, S>(
    sock: S,
) -> impl Transport<Item, SinkItem, TransportError = RpcError>
where
    SinkItem: DeserializeOwned,
    Item: Serialize,
    S: Transport<Vec<u8>, Vec<u8>>,
    RpcError: From<S::Error>,
{
    sock.with(|client_msg| async move { Ok(bincode::serialize(&client_msg)?) })
        .map(|byt| Ok(bincode::deserialize(&byt?)?))
}

#[derive(thiserror::Error, Debug)]
pub enum RpcError {
    #[error("Serialization")]
    Bincode(#[from] BincodeError),
    #[error("Websocket")]
    WebSocket(#[from] Infallible),

    #[cfg(feature = "tokio-tungstenite")]
    #[error("Websocket")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
}
