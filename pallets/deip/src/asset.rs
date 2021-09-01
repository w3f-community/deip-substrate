use crate::*;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Asset<AssetId, AssetBalance> {
    pub id: AssetId,
    pub amount: AssetBalance,
}

impl<AssetId, AssetBalance> Asset<AssetId, AssetBalance> {
    pub fn new(id: AssetId, amount: AssetBalance) -> Self {
        Self { id, amount }
    }
}
