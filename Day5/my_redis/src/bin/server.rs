#![feature(impl_trait_in_assoc_type)]
use my_redis::FliterLayer;
use std::net::SocketAddr;
use my_redis::{S};

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8081".parse().unwrap();
    let addr = volo::net::Address::from(addr);

   
    volo_gen::my_redis::ItemServiceServer::new(S)
    .layer_front(FliterLayer)
    .run(addr)
    .await
    .unwrap();
    
    
    // println!("here");
    // let _ = thread.await;
}
