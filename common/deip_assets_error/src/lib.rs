pub enum ReserveError<AssetId> {
    NotEnoughBalance,
    AssetTransferFailed(AssetId),
}
