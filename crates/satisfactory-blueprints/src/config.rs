use satisfactory_common::types::{FLinearColor, FString};
use scroll::{ctx, Pread, Pwrite};

pub const CURRENT_SAVE_VERSION: [u8; 4] = [0x03, 0x00, 0x00, 0x00];

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BlueprintConfigFile<'b> {
    pub save_version: i32,
    pub blueprint_description: FString<'b>,
    pub icon_id: i32,
    pub color: FLinearColor,
    pub referenced_config_library: Option<FString<'b>>,
    pub icon_library_type: Option<FString<'b>>,
}

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for BlueprintConfigFile<'a> {
    type Error = satisfactory_common::Error;

    fn try_from_ctx(from: &'a [u8], ctx: scroll::Endian) -> Result<(Self, usize), Self::Error> {
        let offset = &mut 0;

        let save_version: i32 = from.gread_with(offset, ctx)?;
        let blueprint_description: FString = from.gread_with(offset, ctx)?;
        let icon_id: i32 = from.gread_with(offset, ctx)?;
        let color: FLinearColor = from.gread_with(offset, ctx)?;

        let mut referenced_config_library: Option<FString> = None;
        let mut icon_library_type: Option<FString> = None;

        // These strings were added in 1.0
        if *offset < from.len() {
            referenced_config_library = Some(from.gread_with(offset, ctx)?);
            icon_library_type = Some(from.gread_with(offset, ctx)?);
        }

        Ok((
            BlueprintConfigFile {
                save_version,
                blueprint_description,
                icon_id,
                color,
                referenced_config_library,
                icon_library_type,
            },
            *offset,
        ))
    }
}

impl<'a> ctx::TryIntoCtx<scroll::Endian> for BlueprintConfigFile<'a> {
    type Error = satisfactory_common::Error;

    fn try_into_ctx(self, this: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;

        this.gwrite_with(CURRENT_SAVE_VERSION, offset, ctx)?;
        this.gwrite_with(self.blueprint_description, offset, ctx)?;
        this.gwrite_with(self.icon_id, offset, ctx)?;
        this.gwrite_with(self.color, offset, ctx)?;

        if let Some(referenced_config_library) = self.referenced_config_library {
            this.gwrite_with(referenced_config_library, offset, ctx)?;
        }

        if let Some(icon_library_type) = self.icon_library_type {
            this.gwrite_with(icon_library_type, offset, ctx)?;
        }

        Ok(*offset)
    }
}
