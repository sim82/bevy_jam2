use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::TypeUuid,
    utils::HashMap,
};
use std::path::Path;

use super::aseprite;

#[derive(Debug, TypeUuid)]
#[uuid = "ab3a0ad8-6fbc-4528-a4a5-90e7bf3fa9e1"]
pub struct Spritesheet {
    pub image: String,
    pub ranges: HashMap<String, std::ops::RangeInclusive<usize>>,
    pub durations: Vec<u64>,
}

impl Spritesheet {
    fn try_from_bytes(_asset_path: &Path, bytes: Vec<u8>) -> Result<Spritesheet> {
        let desc: aseprite::Desc = serde_json::from_slice(&bytes[..]).unwrap();

        println!("desc: {:?}", desc);

        let ranges = desc
            .meta
            .frame_tags
            .iter()
            .map(|tag| (tag.name.clone(), tag.from as usize..=tag.to as usize))
            .collect();

        let durations = desc.frames.iter().map(|f| f.duration as u64).collect();

        let spritesheet = Spritesheet {
            image: "".into(),
            ranges,
            durations,
        };

        Ok(spritesheet)
    }
}

#[derive(Default)]
pub struct SpritesheetLoader {}

impl AssetLoader for SpritesheetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let path = load_context.path();
            let map = Spritesheet::try_from_bytes(path, bytes.into())?;
            load_context.set_default_asset(LoadedAsset::new(map));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["json"];
        EXTENSIONS
    }
}
