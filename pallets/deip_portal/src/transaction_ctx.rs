use deip_transaction_ctx::{ExtrinsicExecCtxT, ExtrinsicExecCtxId, PortalCtxT, ctx_t, ExtrinsicExecCtx};
use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};

ctx_t!(PortalCtx);

impl<T: crate::Config, LocalCall> PortalCtxT<LocalCall> for PortalCtx<ExtrinsicExecCtx<T>>
    where T: SendTransactionTypes<LocalCall>
{
    type OverarchingCall = <T as SendTransactionTypes<LocalCall>>::OverarchingCall;
    type PortalId = T::PortalId;

    fn submit_transaction(call: LocalCall, ctx: ExtrinsicExecCtxId<Self>) {
        let call = crate::Call::on_behalf(Self::portal_id(), Box::new(call.into()));
        SubmitTransaction::<T, LocalCall>::submit_unsigned_transaction(call.into());
    }

    fn portal_id() -> Self::PortalId {
        todo!()
    }
} 
