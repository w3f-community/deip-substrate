pub use jsonrpc_core::{Error as RpcError, ErrorCode as RpcErrorCode};

pub trait GetError {
    fn get_error() -> Error;
}

pub enum Error {
    ScRpcApiError,
    AssetDetailsDecodeFailed,
    AssetIdDecodeFailed,
    AccountIdDecodeFailed,
    AssetBalanceDecodeFailed,
    NoneForReturnedKey,
    DaoDecodeFailed,
    DaoApiGetFailed,
    DaoApiGetMultiFailed,
}

impl Into<RpcErrorCode> for Error {
    fn into(self) -> RpcErrorCode {
        use Error::*;

        const BASE: i64 = 9900;

        RpcErrorCode::ServerError(match self {
            ScRpcApiError => BASE,
            AssetDetailsDecodeFailed => BASE + 1,
            AssetIdDecodeFailed => BASE + 2,
            AccountIdDecodeFailed => BASE + 3,
            AssetBalanceDecodeFailed => BASE + 4,
            NoneForReturnedKey => BASE + 5,
            DaoDecodeFailed => BASE + 6,
            DaoApiGetFailed => BASE + 7,
            DaoApiGetMultiFailed => BASE + 8,
        })
    }
}

pub fn to_rpc_error(e: Error, data: Option<String>) -> RpcError {
    RpcError {
        message: String::new(),
        code: e.into(),
        data: data.map(|d| d.into()),
    }
}
