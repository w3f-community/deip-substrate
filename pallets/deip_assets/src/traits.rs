use crate::*;

pub trait DeipProjectsInfo {
    type ProjectId: sp_std::fmt::Debug + Clone + frame_support::codec::FullCodec + PartialEq;

    fn exists(id: &Self::ProjectId) -> bool;
}
