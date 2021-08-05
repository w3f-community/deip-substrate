pub trait DeipProjectsInfo {
    type ProjectId: sp_std::fmt::Debug + Clone + frame_support::codec::FullCodec + PartialEq + AsRef<[u8]>;

    fn exists(id: &Self::ProjectId) -> bool;
}
