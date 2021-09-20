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
    DomainDecodeFailed,
    DaoIdDecodeFailed,
    DomainIdDecodeFailed,
    ProjectIdDecodeFailed,
    ProjectDecodeFailed,
    InvestmentOpportunityApiGetFailed,
    InvestmentIdDecodeFailed,
    InvestmentOpportunityDecodeFailed,
    AgreementIdDecodeFailed,
    AgreementDecodeFailed,
    AgreementApiGetFailed,
    DomainApiGetFailed,
    ProjectApiGetFailed,
    ProjectContentApiGetFailed,
    ProjectContentIdDecodeFailed,
    ProjectContentDecodeFailed,
    ReviewApiGetFailed,
    ReviewIdDecodeFailed,
    ReviewDecodeFailed,
    UpvoteIdDecodeFailed,
    UpvoteDecodeFailed,
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
            DomainDecodeFailed => BASE + 9,
            DaoIdDecodeFailed => BASE + 10,
            DomainIdDecodeFailed => BASE + 11,
            ProjectIdDecodeFailed => BASE + 12,
            ProjectDecodeFailed => BASE + 13,
            InvestmentOpportunityApiGetFailed => BASE + 14,
            InvestmentIdDecodeFailed => BASE + 15,
            InvestmentOpportunityDecodeFailed => BASE + 16,
            AgreementIdDecodeFailed => BASE + 17,
            AgreementDecodeFailed => BASE + 18,
            AgreementApiGetFailed => BASE + 19,
            DomainApiGetFailed => BASE + 20,
            ProjectApiGetFailed => BASE + 21,
            ProjectContentApiGetFailed => BASE + 22,
            ProjectContentIdDecodeFailed => BASE + 23,
            ProjectContentDecodeFailed => BASE + 24,
            ReviewApiGetFailed => BASE + 25,
            ReviewIdDecodeFailed => BASE + 26,
            ReviewDecodeFailed => BASE + 27,
            UpvoteIdDecodeFailed => BASE + 28,
            UpvoteDecodeFailed => BASE + 29,
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
