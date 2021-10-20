use deip_transaction_ctx::{ExtrinsicExecCtxT, ExtrinsicExecCtxId, PortalCtxT, ctx_t, ExtrinsicExecCtx};

ctx_t!(PortalCtx);

impl<T: frame_system::Config> PortalCtxT for PortalCtx<ExtrinsicExecCtx<T>>
{
    type Call = ();
    type PortalId = ();

    fn submit_transaction(call: Self::Call, ctx: ExtrinsicExecCtxId<Self>) {
        todo!()
    }

    fn portal_id(&self) -> Self::PortalId {
        todo!()
    }
} 
