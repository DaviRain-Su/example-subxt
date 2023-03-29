use codec::{Decode, Encode};
use mmr_rpc::LeavesProof;
use subxt::rpc::types::BlockNumber;
use subxt::{rpc_params, OnlineClient, PolkadotConfig};

// #[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
// #[subxt::subxt(runtime_metadata_url = "wss://rococo-rpc.polkadot.io:443")]
#[subxt::subxt(runtime_metadata_url = "ws://127.0.0.1:9944")]
pub mod polkadot {}

#[derive(codec::Encode, codec::Decode, PartialEq, Eq)]
pub struct EncodableOpaqueLeaf(pub Vec<u8>);

/// Subscribe to all events, and then manually look through them and
/// pluck out the events that we care about.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    println!("-------------------------------------------------------------------");
    // ------------------------ chain_getBlockHash ---------------------
    // get block hash by block number
    let bn = 691u32;
    let bn1 = 690u32;

    let block_numner = Option::<BlockNumber>::Some(bn.into());
    let block_hash: sp_core::H256 = api
        .rpc()
        .request("chain_getBlockHash", rpc_params![block_numner])
        .await?;
    println!("block number ({bn}) block hash is ({block_hash:?})");
    println!("-------------------------------------------------------------------");
    // ------------------------- mmr_root -------------------------------
    let at = Option::<sp_core::H256>::Some(block_hash);
    let root: sp_core::H256 = api.rpc().request("mmr_root", rpc_params![at]).await?;
    println!("at block number ({bn}), mmr root is {root:?}");
    println!("-------------------------------------------------------------------");
    // --------------mmr_generateProof -------------------------------
    let block_numner: Vec<BlockNumber> = vec![bn1.into()];
    let best_known_block_number = Option::<BlockNumber>::Some(bn.into());
    let at = Option::<sp_core::H256>::None;

    let params = rpc_params![block_numner, best_known_block_number, at];
    let leaves_proof_result: LeavesProof<sp_core::H256> =
        api.rpc().request("mmr_generateProof", params).await?;
    println!("at block number ({bn1}), leaves and proof: {leaves_proof_result:?}");
    println!("-------------------------------------------------------------------");
    // -------------- mmr_verifyProofStateless -----------------------
    let mmr_verify_proof_stateless_result: bool = api
        .rpc()
        .request(
            "mmr_verifyProofStateless",
            rpc_params![root, leaves_proof_result.clone()],
        )
        .await?;
    assert!(mmr_verify_proof_stateless_result);
    println!("-------------------------------------------------------------------");
    // -------------- ---------------------------------------
    let encode_leaves: Vec<EncodableOpaqueLeaf> =
        Decode::decode(&mut &leaves_proof_result.leaves[..]).unwrap();
    for item in encode_leaves.into_iter() {
        let leaf = beefy_light_client::mmr::MmrLeaf::decode(&mut &item.0[..]);
        println!("leaf: {leaf:?}");
    }
    println!("-------------------------------------------------------------------");
    let mmr_proof =
        beefy_light_client::mmr::MmrLeafProof::decode(&mut &leaves_proof_result.proof[..]);
    println!("mmr_proof: {mmr_proof:?}");
    println!("-------------------------------------------------------------------");
    Ok(())
}
