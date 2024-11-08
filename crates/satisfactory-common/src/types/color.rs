use scroll::{Pread, Pwrite};

/// Binary representation of an Unreal Engine 5 FLinearColor
#[derive(Debug, Clone, PartialEq, Pread, Pwrite)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FLinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
