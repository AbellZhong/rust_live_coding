use std::sync::Arc;

use anyhow::{Ok, Result};
use dashmap::DashMap;
use tokio::net::TcpListener;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;
use tracing_subscriber;
use futures::Stream;

struct ServerState {
    store: dashmap::DashMap<String, Vec<u8>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            store: DashMap::default(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Server Start!");

    let state = Arc::new(ServerState::new());

    let addr = "0.0.0.0:8888";

    let listener = TcpListener::bind(addr).await?;

    info!("Server listening on {:?}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;

        info!("New Client coming: {:?}", addr);

        // tokio::spawn(async move|| {
        //     let mut stream = LengthDelimitedCodec::builder()
        //         .length_field_length(2)
        //         .new_framed(stream);

        //     while let Some(Ok(buf)) = stream.next().await{
        //         todo!()
        //     }
        // });
    }

}
