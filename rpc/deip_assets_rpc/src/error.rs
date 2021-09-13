use super::*;

pub enum Error {
    ScRpcApiError,
    AssetDetailsDecodeFailed,
    AssetIdDecodeFailed,
    AccountIdDecodeFailed,
    AssetBalanceDecodeFailed,
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
