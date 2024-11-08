use core::str;
use std::fmt::Display;

use scroll::{ctx, Pread, Pwrite};

/// Binary representation of an Unreal Engine 5 FString
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FString<'b> {
    // NOTE: The size is the length of the string with the null-terminator
    pub size: i32,
    pub data: &'b str,
}

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for FString<'a> {
    type Error = crate::Error;

    fn try_from_ctx(src: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let size: i32 = src.gread_with(offset, ctx)?;

        // TODO: Parse UTF-16
        //       when size is below 0, parse as utf-16.
        //       I would have implemented it with `try_gread_vec_with` macro but that heap allocates
        //       and I do not want that and scroll does not implement `TryFromCtx` for `&'a [u16]`.

        let data: &[u8] = src.gread_with(offset, (size - 1) as usize)?;
        let data = str::from_utf8(data).map_err(crate::Error::InvalidUTF8)?;

        Ok((FString { size, data }, *offset + 1))
    }
}

impl ctx::TryIntoCtx<scroll::Endian> for FString<'_> {
    type Error = scroll::Error;

    fn try_into_ctx(self, dst: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        dst.gwrite_with(self.size, offset, ctx)?;
        dst.gwrite_with(self.data.as_bytes(), offset, ())?;
        dst.gwrite_with(b'\0', offset, ctx)?; // Add the missing null-terminator

        Ok(*offset)
    }
}

impl From<FString<'_>> for String {
    fn from(value: FString) -> Self {
        value.data.to_owned()
    }
}

impl Display for FString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
