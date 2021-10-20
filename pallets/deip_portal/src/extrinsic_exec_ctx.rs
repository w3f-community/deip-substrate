use frame_system::Pallet as System;

/// Context of a transaction that executed in
pub trait ExtrinsicExecCtxT: Sized {
    type BlockNumber;
    type ExtrinsicId;
    
    fn new() -> Self;
    
    fn block_number(&self) -> Self::BlockNumber;
    fn extrinsic_id(&self) -> Self::ExtrinsicId;
    fn id(&self) -> ExtrinsicExecCtxId<Self>;
}

pub struct ExtrinsicExecCtx<T>(std::marker::PhantomData<T>);
impl<T: frame_system::Config> ExtrinsicExecCtxT
    for ExtrinsicExecCtx<T>
{
    type BlockNumber = T::BlockNumber;
    type ExtrinsicId = u32;

    fn new() -> Self { Self(Default::default()) }
    
    fn block_number(&self) -> Self::BlockNumber {
        System::<T>::block_number()
    }

    fn extrinsic_id(&self) -> Self::ExtrinsicId {
        System::<T>::extrinsic_index().unwrap()
    }

    fn id(&self) -> ExtrinsicExecCtxId<Self> {
        ExtrinsicExecCtxId {
            block_number: self.block_number(),
            extrinsic_id: self.extrinsic_id()
        }
    }
}

/// Id of a context that transaction executed in
pub struct ExtrinsicExecCtxId<Ctx: ExtrinsicExecCtxT + ?Sized> {
    pub block_number: Ctx::BlockNumber,
    pub extrinsic_id: Ctx::ExtrinsicId
}
