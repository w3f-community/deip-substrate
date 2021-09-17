use codec::Decode;
use common_rpc::*;

// Domains

pub struct DomainIdError;
impl GetError for DomainIdError {
    fn get_error() -> Error {
        Error::DomainIdDecodeFailed
    }
}

pub struct DomainError;
impl GetError for DomainError {
    fn get_error() -> Error {
        Error::DomainDecodeFailed
    }
}

pub struct DomainKeyValue {
    pub id: super::DomainId,
}

impl DomainKeyValue {
    pub fn new(id: super::DomainId) -> Self {
        Self { id }
    }
}

impl KeyValueInfo for DomainKeyValue {
    type Key = super::DomainId;
    type KeyError = DomainIdError;
    type Value = super::Domain;
    type ValueError = DomainError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Projects

pub struct ProjectIdError;
impl GetError for ProjectIdError {
    fn get_error() -> Error {
        Error::ProjectIdDecodeFailed
    }
}

pub struct ProjectError;
impl GetError for ProjectError {
    fn get_error() -> Error {
        Error::ProjectDecodeFailed
    }
}

pub struct ProjectKeyValue<Hash, AccountId> {
    pub id: super::ProjectId,
    _m: std::marker::PhantomData<(Hash, AccountId)>,
}

impl<Hash, AccountId> ProjectKeyValue<Hash, AccountId> {
    pub fn new(id: super::ProjectId) -> Self {
        Self {
            id,
            _m: Default::default(),
        }
    }
}

impl<Hash: 'static + Decode + Send, AccountId: 'static + Decode + Send> KeyValueInfo
    for ProjectKeyValue<Hash, AccountId>
{
    type Key = super::ProjectId;
    type KeyError = ProjectIdError;
    type Value = super::Project<Hash, AccountId>;
    type ValueError = ProjectError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Investment opportunities

pub struct InvestmentIdError;
impl GetError for InvestmentIdError {
    fn get_error() -> Error {
        Error::InvestmentIdDecodeFailed
    }
}

pub struct InvestmentOpportunityError;
impl GetError for InvestmentOpportunityError {
    fn get_error() -> Error {
        Error::InvestmentOpportunityDecodeFailed
    }
}

pub struct InvestmentOpportunityKeyValue<Moment, AssetId, AssetBalance> {
    pub id: super::InvestmentId,
    _m: std::marker::PhantomData<(Moment, AssetId, AssetBalance)>,
}

impl<Moment, AssetId, AssetBalance> InvestmentOpportunityKeyValue<Moment, AssetId, AssetBalance> {
    pub fn new(id: super::InvestmentId) -> Self {
        Self {
            id,
            _m: Default::default(),
        }
    }
}

impl<
        Moment: 'static + Decode + Send,
        AssetId: 'static + Decode + Send,
        AssetBalance: 'static + Decode + Send,
    > KeyValueInfo for InvestmentOpportunityKeyValue<Moment, AssetId, AssetBalance>
{
    type Key = super::InvestmentId;
    type KeyError = InvestmentIdError;
    type Value = super::SimpleCrowdfunding<Moment, AssetId, AssetBalance>;
    type ValueError = InvestmentOpportunityError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Contract agreements

pub struct AgreementIdError;
impl GetError for AgreementIdError {
    fn get_error() -> Error {
        Error::AgreementIdDecodeFailed
    }
}

pub struct AgreementError;
impl GetError for AgreementError {
    fn get_error() -> Error {
        Error::AgreementDecodeFailed
    }
}

pub struct AgreementKeyValue<AccountId, Hash, Moment, AssetId, AssetBalance> {
    pub id: super::ContractAgreementId,
    _m: std::marker::PhantomData<(AccountId, Hash, Moment, AssetId, AssetBalance)>,
}

impl<AccountId, Hash, Moment, AssetId, AssetBalance>
    AgreementKeyValue<AccountId, Hash, Moment, AssetId, AssetBalance>
{
    pub fn new(id: super::ContractAgreementId) -> Self {
        Self {
            id,
            _m: Default::default(),
        }
    }
}

impl<AccountId, Hash, Moment, AssetId, AssetBalance> KeyValueInfo
    for AgreementKeyValue<AccountId, Hash, Moment, AssetId, AssetBalance>
where
    AccountId: 'static + Decode + Send,
    Hash: 'static + Decode + Send,
    Moment: 'static + Decode + Send,
    AssetId: 'static + Decode + Send,
    AssetBalance: 'static + Decode + Send,
{
    type Key = super::ContractAgreementId;
    type KeyError = AgreementIdError;
    type Value = super::contract::Agreement<
        AccountId,
        Hash,
        Moment,
        super::DeipAsset<AssetId, AssetBalance>,
    >;
    type ValueError = AgreementError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Project contents

pub struct ProjectContentIdError;
impl GetError for ProjectContentIdError {
    fn get_error() -> Error {
        Error::ProjectContentIdDecodeFailed
    }
}

pub struct ProjectContentError;
impl GetError for ProjectContentError {
    fn get_error() -> Error {
        Error::ProjectContentDecodeFailed
    }
}

pub struct ProjectContentKeyValue<Hash, AccountId> {
    pub id: super::ProjectContentId,
    _m: std::marker::PhantomData<(Hash, AccountId)>,
}

impl<Hash, AccountId> ProjectContentKeyValue<Hash, AccountId> {
    pub fn new(id: super::ProjectContentId) -> Self {
        Self {
            id,
            _m: Default::default(),
        }
    }
}

impl<Hash, AccountId> KeyValueInfo for ProjectContentKeyValue<Hash, AccountId>
where
    AccountId: 'static + Decode + Send,
    Hash: 'static + Decode + Send,
{
    type Key = super::ProjectContentId;
    type KeyError = ProjectContentIdError;
    type Value = super::ProjectContent<Hash, AccountId>;
    type ValueError = ProjectContentError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}

// Reviews

pub struct ReviewIdError;
impl GetError for ReviewIdError {
    fn get_error() -> Error {
        Error::ReviewIdDecodeFailed
    }
}

pub struct ReviewError;
impl GetError for ReviewError {
    fn get_error() -> Error {
        Error::ReviewDecodeFailed
    }
}

pub struct ReviewKeyValue<Hash, AccountId> {
    pub id: super::ReviewId,
    _m: std::marker::PhantomData<(Hash, AccountId)>,
}

impl<Hash, AccountId> ReviewKeyValue<Hash, AccountId> {
    pub fn new(id: super::ReviewId) -> Self {
        Self {
            id,
            _m: Default::default(),
        }
    }
}

impl<Hash, AccountId> KeyValueInfo for ReviewKeyValue<Hash, AccountId>
where
    AccountId: 'static + Decode + Send,
    Hash: 'static + Decode + Send,
{
    type Key = super::ReviewId;
    type KeyError = ReviewIdError;
    type Value = super::Review<Hash, AccountId>;
    type ValueError = ReviewError;

    fn key(&self) -> &Self::Key {
        &self.id
    }
}
