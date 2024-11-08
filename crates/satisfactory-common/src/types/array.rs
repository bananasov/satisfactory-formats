use crate::try_gread_vec_with;
use scroll::{ctx, Pread};

// TODO: Writing arrays
// TODO: Implement more types

/// Binary representation of an Unreal Engine 5 TArray
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TArray<T> {
    pub len: i32,
    pub data: Vec<T>,
}

impl<T> From<TArray<T>> for Vec<T> {
    fn from(value: TArray<T>) -> Self {
        value.data
    }
}

impl<'a, T: 'a> ctx::TryFromCtx<'a, scroll::Endian> for TArray<T>
where
    T: ctx::TryFromCtx<'a, scroll::Endian, Error = crate::Error>,
{
    type Error = crate::Error;

    fn try_from_ctx(from: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let len: i32 = from.gread_with(offset, ctx)?;
        let data: Vec<T> = try_gread_vec_with!(from, offset, len as usize, ctx);

        Ok((TArray { len, data }, *offset))
    }
}
