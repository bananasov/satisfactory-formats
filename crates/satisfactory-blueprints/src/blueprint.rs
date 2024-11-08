use satisfactory_common::types::{FString, TArray, Vector3};
use scroll::{ctx, Pread};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RecipeReference<'b> {
    pub object_type: i32,
    pub object_name: FString<'b>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ItemCost<'b> {
    pub object_type: i32,
    pub object_name: FString<'b>,
    pub count: i32,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BlueprintFileHeader<'b> {
    pub save_version: i32,
    pub build_version: i32,
    pub dimensions: Vector3<i32>,
    pub item_costs: Vec<ItemCost<'b>>, // Maybe use a HashMap?
    pub recipe_references: Vec<RecipeReference<'b>>,
}

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for BlueprintFileHeader<'a> {
    type Error = satisfactory_common::Error;

    fn try_from_ctx(src: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let save_version: i32 = src.gread_with(offset, ctx)?;
        let build_version: i32 = src.gread_with(offset, ctx)?;
        let _ = src.gread_with::<i32>(offset, ctx)?; // I don't know what this value means.

        let dimensions: Vector3<i32> = src.gread_with(offset, ctx)?;
        let costs: TArray<ItemCost> = src.gread_with(offset, ctx)?;
        let recipe_references: TArray<RecipeReference> = src.gread_with(offset, ctx)?;

        Ok((
            BlueprintFileHeader {
                save_version,
                build_version,
                dimensions,
                item_costs: costs.into(),
                recipe_references: recipe_references.into(),
            },
            *offset,
        ))
    }
}

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for ItemCost<'a> {
    type Error = satisfactory_common::Error;

    fn try_from_ctx(from: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let object_type: i32 = from.gread_with(offset, ctx)?;
        let object_name: FString = from.gread_with(offset, ctx)?;
        let count: i32 = from.gread_with(offset, ctx)?;

        Ok((
            ItemCost {
                object_type,
                object_name,
                count,
            },
            *offset,
        ))
    }
}

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for RecipeReference<'a> {
    type Error = satisfactory_common::Error;

    fn try_from_ctx(from: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let object_type: i32 = from.gread_with(offset, ctx)?;
        let object_name: FString = from.gread_with(offset, ctx)?;

        Ok((
            RecipeReference {
                object_type,
                object_name,
            },
            *offset,
        ))
    }
}
