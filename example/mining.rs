use std::path::Path;

use anyhow::Ok;
use bitcoin::{block::Header, consensus::Decodable};
use bitcoin_ipc::init::setup_connection;
use tokio::task::LocalSet;

const BLOCK_RESERVED_WEIGHT: u64 = 4_000_000;

macro_rules! send {
    ($expr:expr) => {
        $expr.send().promise.await?.get()?.get_result()
    };
}

macro_rules! set_thread {
    ($client:expr, $thread:expr) => {
        $client.get().get_context()?.set_thread($thread)
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).expect("Path to Unix socket required");

    let local = LocalSet::new();
    local
        .run_until(async move {
            let path = Path::new(&path);
            let (init, thread) = setup_connection(path).await.unwrap();
            let mut mining_req = init.make_mining_request();
            set_thread!(mining_req, thread.clone());
            let mining_client = send!(mining_req)?;
            let mut is_test_mining = mining_client.is_test_chain_request();
            set_thread!(is_test_mining, thread.clone());
            let is_test_chain = send!(is_test_mining);
            println!("Is this a test chain: {is_test_chain}");
            let mut create_block_req = mining_client.create_new_block_request();
            create_block_req
                .get()
                .get_options()?
                .set_block_reserved_weight(BLOCK_RESERVED_WEIGHT);
            let create_block_client = send!(create_block_req)?;
            let mut header_req = create_block_client.get_block_header_request();
            set_thread!(header_req, thread);
            let header_res = header_req.send().promise.await?;
            let mut header = header_res.get()?.get_result()?;
            let deserialized_header = Header::consensus_decode(&mut header)?;
            println!("Block header: {:?}", deserialized_header);
            Ok(())
        })
        .await?;
    Ok(())
}
