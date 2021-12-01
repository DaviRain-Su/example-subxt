use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    EventSubscription,
    PairSigner,
};

mod example;
pub use example::*;

#[subxt::subxt(
    runtime_metadata_path = "metadata/metadata.scale",
    generated_type_derives = "Clone, Debug"
)]
pub mod polkadot {}

// can test
// use polkadot::runtime_types::frame_support::PalletId;
//
// #[async_std::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let pallet_id = PalletId([1u8; 8]);
//     let _ = <PalletId as Clone>::clone(&pallet_id);
//     Ok(())
// }

// can test
// #[async_std::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     env_logger::init();
//
//     let api = ClientBuilder::new()
//         .build()
//         .await?
//         .to_runtime_api::<polkadot::RuntimeApi<polkadot::DefaultConfig>>();
//
//     let mut iter = api.storage().system().account_iter(None).await?;
//
//     while let Some((key, account)) = iter.next().await? {
//         println!("{}: {}", hex::encode(key), account.data.free);
//     }
//     Ok(())
// }

// can pass
// #[async_std::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     env_logger::init();
//
//     let api = ClientBuilder::new()
//         .set_url("ws://127.0.0.1:9944")
//         .build()
//         .await?
//         .to_runtime_api::<polkadot::RuntimeApi<polkadot::DefaultConfig>>();
//
//     let block_number = 1;
//
//     let block_hash = api
//         .client
//         .rpc()
//         .block_hash(Some(block_number.into()))
//         .await?;
//
//     if let Some(hash) = block_hash {
//         println!("Block hash for block number {}: {}", block_number, hash);
//     } else {
//         println!("Block number {} not found.", block_number);
//     }
//
//     Ok(())
// }

// cannot pass

// #[async_std::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     env_logger::init();
//
//     let signer = PairSigner::new(AccountKeyring::Dave.pair());
//     let dest = AccountKeyring::Eve.to_account_id().into();
//
//     let api = ClientBuilder::new()
//         // .set_url("ws://127.0.0.1:9944")
//         .build()
//         .await?
//         .to_runtime_api::<polkadot::RuntimeApi<polkadot::DefaultConfig>>();
//
//     let hash = api
//         .tx()
//         .balances()
//         .transfer(dest, 10_000)
//         .sign_and_submit(&signer)
//         .await?;
//
//     println!("Balance transfer extrinsic submitted: {}", hash);
//
//     Ok(())
// }

// cannot pass

// #[async_std::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     env_logger::init();
//
//     let signer = PairSigner::new(AccountKeyring::Alice.pair());
//     let dest = AccountKeyring::Bob.to_account_id().into();
//
//     let api = ClientBuilder::new()
//         .build()
//         .await?
//         .to_runtime_api::<polkadot::RuntimeApi<polkadot::DefaultConfig>>();
//     let result = api
//         .tx()
//         .balances()
//         .transfer(dest, 10_000)
//         .sign_and_submit_then_watch(&signer)
//         .await?;
//
//     if let Some(event) = result.find_event::<polkadot::balances::events::Transfer>()? {
//         println!("Balance transfer success: value: {:?}", event.2);
//     } else {
//         println!("Failed to find Balances::Transfer Event");
//     }
//     Ok(())
// }
//
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();
    println!("dest: {}", dest);

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<polkadot::DefaultConfig>>();

    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<polkadot::DefaultConfig>::new(sub, decoder);
    sub.filter_event::<polkadot::balances::events::Transfer>();

    api.tx()
        .balances()
        .transfer(dest, 10_000)
        .sign_and_submit(&signer)
        .await?;

    let raw = sub.next().await.unwrap().unwrap();
    let event = <polkadot::balances::events::Transfer as codec::Decode>::decode(
        &mut &raw.data[..],
    );
    if let Ok(e) = event {
        println!("Balance transfer success: value: {:?}", e.2);
    } else {
        println!("Failed to subscribe to Balances::Transfer Event");
    }
    Ok(())
}

