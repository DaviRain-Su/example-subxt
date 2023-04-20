use codec::{Decode, Encode};
use mmr_rpc::LeavesProof;
use serde::{Deserialize, Serialize};
use subxt::rpc::types::BlockNumber;
use subxt::rpc::RpcClient;
use subxt::rpc::Subscription;
use subxt::{rpc_params, OnlineClient, PolkadotConfig};

// #[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
// #[subxt::subxt(runtime_metadata_url = "wss://rococo-rpc.polkadot.io:443")]
#[subxt::subxt(runtime_metadata_url = "ws://127.0.0.1:9944")]
pub mod polkadot {}

#[derive(codec::Encode, codec::Decode, PartialEq, Eq)]
pub struct EncodableOpaqueLeaf(pub Vec<u8>);

/// An encoded signed commitment proving that the given header has been finalized.
/// The given bytes should be the SCALE-encoded representation of a
/// `beefy_primitives::SignedCommitment`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignedCommitment(pub sp_core::Bytes);

/// Subscribe to beefy justifications.
pub async fn subscribe_beefy_justifications(
    client: &RpcClient,
) -> Result<Subscription<SignedCommitment>, subxt::Error> {
    let subscription = client
        .subscribe(
            "beefy_subscribeJustifications",
            rpc_params![],
            "beefy_unsubscribeJustifications",
        )
        .await?;
    Ok(subscription)
}

/// Subscribe to all events, and then manually look through them and
/// pluck out the events that we care about.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    // subscribe beefy justification and get signed commitment
    let mut sub = subscribe_beefy_justifications(&*api.rpc()).await.unwrap();

    while let Ok(raw_signed_commitment) = sub.next().await.unwrap() {
        // decode signed commitment
        let beefy_light_client::commitment::VersionedFinalityProof::V1(signed_commitment) =
            beefy_light_client::commitment::VersionedFinalityProof::decode(
                &mut &raw_signed_commitment.0[..],
            )
            .unwrap();

        // get commitment
        let beefy_light_client::commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commitment.commitment.clone();

        println!("-------------------------------------------------------------------");
        // ------------------------ chain_getBlockHash ---------------------
        // get block hash by block number
        let bn = block_number;
        let bn1 = block_number - 1;

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
        let mmr_leafs =
            beefy_light_client::mmr::decode_mmr_leaves(leaves_proof_result.leaves[..].to_vec());
        println!("mmr_leafs: {mmr_leafs:?}");
        println!("-------------------------------------------------------------------");
        let mmr_proof = beefy_light_client::mmr::MmrLeavesProof::try_from(
            leaves_proof_result.proof[..].to_vec(),
        );
        println!("mmr_proof: {mmr_proof:?}");
        println!("-------------------------------------------------------------------");
    }

    Ok(())
}
