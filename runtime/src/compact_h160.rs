use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

// for details please check parity_scale_codec/src/compact.rs

#[derive(Default, Clone, Copy, sp_runtime::RuntimeDebug, PartialEq, Eq, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct H160(sp_core::H160);

#[derive(Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub struct Compact<T>(pub T);

impl From<Compact<H160>> for H160 {
    fn from(x: Compact<H160>) -> Self {
        H160(x.0 .0)
    }
}

impl From<H160> for Compact<H160> {
    fn from(x: H160) -> Self {
        Compact(x)
    }
}

impl<'a> From<&'a H160> for Compact<H160> {
    fn from(x: &'a H160) -> Self {
        Compact(*x)
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct CompactRef<'a, T>(pub &'a T);

impl<'a> From<&'a Compact<H160>> for CompactRef<'a, H160> {
    fn from(x: &'a Compact<H160>) -> Self {
        CompactRef(&x.0)
    }
}

impl<'a> From<&'a H160> for CompactRef<'a, H160> {
    fn from(x: &'a H160) -> Self {
        CompactRef(x)
    }
}

impl codec::EncodeLike for Compact<H160> where for<'a> CompactRef<'a, H160>: codec::Encode {}

impl codec::Encode for Compact<H160>
where
    for<'a> CompactRef<'a, H160>: codec::Encode,
{
    fn size_hint(&self) -> usize {
        CompactRef::<H160>(&self.0).size_hint()
    }

    fn encode_to<W: codec::Output + ?Sized>(&self, dest: &mut W) {
        CompactRef::<H160>(&self.0).encode_to(dest)
    }

    fn encode(&self) -> Vec<u8> {
        CompactRef::<H160>(&self.0).encode()
    }

    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        CompactRef::<H160>(&self.0).using_encoded(f)
    }
}

impl<'a> codec::EncodeAsRef<'a, H160> for Compact<H160>
where
    CompactRef<'a, H160>: codec::Encode + From<&'a H160>,
{
    type RefType = CompactRef<'a, H160>;
}

impl<'a> codec::Encode for CompactRef<'a, H160> {
    fn size_hint(&self) -> usize {
        <[u8; 20] as codec::Encode>::size_hint(&self.0 .0 .0)
    }

    fn encode_to<W: codec::Output + ?Sized>(&self, dest: &mut W) {
        <[u8; 20] as codec::Encode>::encode_to(&self.0 .0 .0, dest)
    }
}

impl codec::Decode for Compact<H160> {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let array = <[u8; 20] as codec::Decode>::decode(input)?;
        Ok(Compact::<H160>(H160(sp_core::H160(array))))
    }
}

impl codec::HasCompact for H160 {
    type Type = Compact<H160>;
}
