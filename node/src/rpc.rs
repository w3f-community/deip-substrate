//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use node_template_runtime::{opaque::Block, AccountId, AssetBalance, AssetId, Balance, Index, Moment, Hash};
pub use sc_rpc_api::DenyUnsafe;
use sp_api::{CallApiAt, Metadata, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_transaction_pool::TransactionPool;

use jsonrpc_pubsub::manager::SubscriptionManager;
use sc_client_api::{BlockchainEvents, ExecutorProvider, ProofProvider, StorageProvider};
use sc_rpc::SubscriptionTaskExecutor;
use sp_runtime::traits;

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// the executor to use with RPC subscriptions
    pub task_executor: SubscriptionTaskExecutor,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, Backend>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: ExecutorProvider<Block>,
    C: StorageProvider<Block, Backend>,
    C: ProofProvider<Block>,
    C: BlockchainEvents<Block>,
    C: CallApiAt<Block, Error = BlockChainError>,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: deip_rpc::DeipStorageRuntimeApi<Block, AccountId, Moment, AssetId, AssetBalance, Hash>,
    C::Api: deip_org_rpc::DeipOrgRuntimeApi<Block, AccountId>,
    C::Api: BlockBuilder<Block>,
    C::Api: Metadata<Block, Error = BlockChainError>,
    P: TransactionPool + 'static,
    Backend: 'static + sc_client_api::backend::Backend<Block> + Send,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
    use substrate_frame_rpc_system::{FullSystem, SystemApi};

    let mut io = jsonrpc_core::IoHandler::default();
    let FullDeps {
        client,
        pool,
        deny_unsafe,
        task_executor,
    } = deps;

    io.extend_with(SystemApi::to_delegate(FullSystem::new(
        client.clone(),
        pool,
        deny_unsafe,
    )));

    io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
        client.clone(),
    )));

    let subscriptions = SubscriptionManager::new(Arc::new(task_executor.clone()));
    let (state, _) = sc_rpc::state::new_full(client.clone(), subscriptions, deny_unsafe);

    // Add a silly RPC that returns constant values
    io.extend_with(deip_rpc::DeipStorageApi::to_delegate(
        deip_rpc::DeipStorage::new(client.clone(), state),
    ));

    let subscriptions = SubscriptionManager::new(Arc::new(task_executor.clone()));
    let (state, _) = sc_rpc::state::new_full(client.clone(), subscriptions, deny_unsafe);

    io.extend_with(deip_org_rpc::DeipOrgRpcApi::to_delegate(
        deip_org_rpc::DeipOrgRpcApiObj::new(client.clone(), state),
    ));

    let subscriptions = SubscriptionManager::new(Arc::new(task_executor.clone()));
    let (state, _) = sc_rpc::state::new_full(client, subscriptions, deny_unsafe);

    io.extend_with(deip_assets_rpc::DeipAssetsRpc::<
        <Block as traits::Block>::Hash,
        AssetId,
        AssetBalance,
        AccountId,
        Balance,
    >::to_delegate(deip_assets_rpc::DeipAssetsRpcObj::<
        sc_rpc::state::State<Block, C>,
        Block,
    >::new(state)));

    // Extend this RPC with a custom API by using the following syntax.
    // `YourRpcStruct` should have a reference to a client, which is needed
    // to call into the runtime.
    // `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

    io
}
