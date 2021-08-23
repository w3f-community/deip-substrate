pub enum ReserveError<AssetId> {
    NotEnoughBalance,
    AlreadyReserved,
    AssetTransferFailed(AssetId),
}

pub enum UnreserveError<AssetId> {
    NoSuchInvestment,
    AssetTransferFailed(AssetId),
}
