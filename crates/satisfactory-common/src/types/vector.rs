use scroll::{Pread, Pwrite};

/// Binary representation of an Unreal Engine 5 IntVector2
#[derive(Debug, Clone, Copy, PartialEq, Pread, Pwrite)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

/// Binary representation of an Unreal Engine 5 IntVector3
#[derive(Debug, Clone, Copy, PartialEq, Pread, Pwrite)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Binary representation of an Unreal Engine 5 IntVector4
#[derive(Debug, Clone, Copy, PartialEq, Pread, Pwrite)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
