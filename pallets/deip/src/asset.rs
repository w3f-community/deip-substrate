use crate::*;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Asset<AssetId, AssetBalance> {
    pub id: AssetId,
    pub amount: AssetBalance,
}
