use codec::Decode;

pub struct DaoError;
impl common_rpc::GetError for DaoError {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::DaoDecodeFailed
    }
}

pub struct DaoIdError;
impl common_rpc::GetError for DaoIdError {
    fn get_error() -> common_rpc::Error {
        common_rpc::Error::DaoIdDecodeFailed
    }
}

pub struct DaoKeyValue<AccountId, Id> {
    pub id: super::OrgName,
    _m: std::marker::PhantomData<(AccountId, Id)>,
}

impl<AccountId, Id> DaoKeyValue<AccountId, Id> {
    pub fn new(id: super::OrgName) -> Self {
        Self {
            id,
            _m: Default::default(),
        }
    }
}

impl<AccountId: 'static + Decode + Send, Id: 'static + Decode + Send> common_rpc::KeyValueInfo
    for DaoKeyValue<AccountId, Id>
{
    type Key = super::OrgName;
    type KeyError = DaoIdError;
    type Value = super::Org<AccountId, Id>;
    type ValueError = DaoError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}
