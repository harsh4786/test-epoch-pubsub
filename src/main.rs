use solana_client::{
    nonblocking::{
        pubsub_client::{self, PubsubClient}, rpc_client::RpcClient,
    },
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig, RpcSendTransactionConfig, RpcSimulateTransactionAccountsConfig},
    rpc_filter::{Memcmp, RpcFilterType},
    rpc_response::EpochUpdates,
};

use futures_util::StreamExt;

#[tokio::main]
async fn main() {
   let pubsub_client = PubsubClient::new("ws://127.0.0.1:8900").await;

   match pubsub_client{
    Ok(client) => {
        match client.epochs_updates_subscribe().await {
            Ok((mut stream, _unsubscribe)) => {
                while let Some(e) = stream.next().await {
                    println!("Response: {:?}", e);
                }
                print!("nothing here...1");
            }
            Err(e) => {
                println!("Error: {:?}", e);
                print!("nothing here...2");
            }
        } 
    }
    Err(e) => {
        println!("Error: {:?}", e);
        print!("nothing here...3");
    }
   }
}
