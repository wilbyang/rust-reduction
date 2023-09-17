pub mod snazzy {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
    }
}

use anyhow::Result;
use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use prost::Message;
use snazzy::items;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};



#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8083";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);
        tokio::spawn(async move {
            
            let framed = Framed::new(stream, LengthDelimitedCodec::new());
            // split 成 writer 和 reader
            let (mut w, mut r) = framed.split();
            while let Some(Ok(shirtb)) = r.next().await {
                let shirt = items::SkuItem::decode(shirtb)?;
                println!("shirt: {:?}", shirt);


                let mut shirt = items::Shirt::default();
                shirt.color = "color".to_string();
                shirt.set_size(items::shirt::Size::Large);
                // use bytes crate to create a buffer
                let mut buf = BytesMut::new();

                // to protobuf bytes
                items::Shirt::encode(&shirt, &mut buf)?;
                w.send(buf.freeze()).await?;
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}
