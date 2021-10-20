use super::ExtrinsicExecCtxT;
use super::ExtrinsicExecCtxId;
// use super::ExtrinsicExecCtx;

pub trait PortalCtxT: ExtrinsicExecCtxT {
    type Call;
    type PortalId;
    
    fn submit_transaction(call: Self::Call, ctx: ExtrinsicExecCtxId<Self>);
    
    fn portal_id(&self) -> Self::PortalId;
}

// pub struct PortalCtx<T, U: ExtrinsicExecCtxT>(U, sp_std::marker::PhantomData<T>);
// 
// impl<T, U> ExtrinsicExecCtxT for PortalCtx<T, U>
//     where T: frame_system::Config, U: ExtrinsicExecCtxT
// {
//     type BlockNumber = U::BlockNumber;
//     type ExtrinsicId = U::ExtrinsicId;
// 
//     fn current() -> Self {
//         Self(U::current(), Default::default())
//     }
// 
//     fn block_number(&self) -> Self::BlockNumber {
//         self.0.block_number()
//     }
// 
//     fn extrinsic_id(&self) -> Self::ExtrinsicId {
//         self.0.extrinsic_id()
//     }
// 
//     fn id(&self) -> ExtrinsicExecCtxId<Self> {
//         let ExtrinsicExecCtxId {
//             block_number, extrinsic_id
//         } = self.0.id();
//         ExtrinsicExecCtxId { block_number, extrinsic_id }
//     }
// }
