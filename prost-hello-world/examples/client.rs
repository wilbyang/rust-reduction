use anyhow::Result;
use bytes::{Bytes, BytesMut};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use prost::Message;
pub mod snazzy {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
    }
}
use snazzy::items;
#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

    let mut shirt = items::Shirt::default();
    shirt.color = "color".to_string();
    shirt.set_size(items::shirt::Size::Large);
    // use bytes crate to create a buffer
    let mut buf = BytesMut::new();

    // to protobuf bytes
    items::Shirt::encode(&shirt, &mut buf)?;
    stream.send(buf.freeze()).await?;

    // 接收从服务器返回的数据
    if let Some(Ok(data)) = stream.next().await {
        let shirt = items::Shirt::decode(data)?;
        println!("shirt: {:?}", shirt);
    }

    Ok(())
}
