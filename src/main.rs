
// use sp_keyring::AccountKeyring;
use subxt::{
    // tx::PairSigner,
    OnlineClient,
    PolkadotConfig, rpc_params,
};
// use std::time::Duration;
// use futures::StreamExt;
use subxt::rpc::types::BlockNumber;
use mmr_rpc::LeavesProof;

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
// #[subxt::subxt(runtime_metadata_url = "wss://rococo-rpc.polkadot.io:443")]
pub mod polkadot {}

/// Subscribe to all events, and then manually look through them and
/// pluck out the events that we care about.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let address = polkadot::storage().system().account_root();

    let mut iter = api.storage().at(None).await?.iter(address, 10).await?;

    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account.data.free);
    }

    // test beefy_getFinalizedHead
    let header_result: sp_core::H256  = api.rpc().request("beefy_getFinalizedHead", rpc_params![]).await?;
    println!("beefy_getFinalizedHead is {:?}", header_result);
    

    // test mmr_generateProof
    let method = "mmr_generateProof";
    // let block_numner: Vec<BlockNumber> = vec![0u32.into(),1u32.into(), 2u32.into()];
    // let block_numner: Vec<BlockNumber> = vec![4007966u32.into(),4007967u32.into(),4007968u32.into()];
    let block_numner: Vec<BlockNumber> = vec![30u64.into()];
    let best_known_block_number = Option::<BlockNumber>::None;
    let at = Option::<sp_core::H256>::None;
    let params = rpc_params![block_numner, best_known_block_number, at];
    let result: LeavesProof<sp_core::H256> = api.rpc().request(method, params).await?;
    println!("mmr_generateProof is {:?}", result);


    Ok(())
}