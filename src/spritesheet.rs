use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
use std::{collections::HashMap, path::Path, time::Duration};

mod aseprite {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Frame {
        // filename : String,
        pub duration: u32,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FrameTag {
        pub name: String,
        pub from: u32,
        pub to: u32,
        pub direction: String,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Meta {
        pub app: String,
        pub version: String,
        pub image: String,
        pub format: String,
        pub scale: String,
        pub frame_tags: Vec<FrameTag>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Desc {
        pub frames: Vec<Frame>,
        pub meta: Meta,
    }
}

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

#[derive(Component)]
pub struct SpritesheetAnimation {
    spritesheet: Handle<Spritesheet>,
    pub active_animation: String,
    current_frame: Option<usize>,
    frame_timer: Option<Timer>,
    do_loop: bool,
    end_frame: bool,
}
impl SpritesheetAnimation {
    pub fn new(spritesheet: Handle<Spritesheet>) -> Self {
        Self {
            spritesheet,
            active_animation: default(),
            current_frame: None,
            frame_timer: None,
            do_loop: false,
            end_frame: false,
        }
    }

    pub fn start_animation(&mut self, name: &str, do_loop: bool) {
        debug!("animations: {} -> {}", self.active_animation, name);
        self.active_animation = name.into();
        self.current_frame = None;
        self.frame_timer = None;
        self.do_loop = do_loop;
        self.end_frame = false;
    }
    pub fn is_animation_finished(&self) -> bool {
        self.end_frame
    }
}
fn spritesheet_animation_system(
    spritesheets: Res<Assets<Spritesheet>>,
    time: Res<Time>,
    mut query: Query<(&mut SpritesheetAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut spritesheet_animation, mut texture_atlas_sprite) in &mut query {
        let has_current_frame = spritesheet_animation.current_frame.is_some();
        if let Some(frame_timer) = spritesheet_animation.frame_timer.as_mut() {
            frame_timer.tick(time.delta());

            if has_current_frame && !frame_timer.just_finished() {
                continue;
            }
        }
        spritesheet_animation.frame_timer = None;

        let spritesheet = spritesheets
            .get(&spritesheet_animation.spritesheet)
            .unwrap();

        let range = spritesheet
            .ranges
            .get(&spritesheet_animation.active_animation)
            .unwrap();
        spritesheet_animation.current_frame = match spritesheet_animation.current_frame {
            Some(mut current_frame) => {
                if current_frame == *range.end() {
                    if spritesheet_animation.do_loop {
                        current_frame = *range.start();
                    }
                    spritesheet_animation.end_frame = true;
                } else {
                    current_frame += 1;
                }
                // if !range.contains(&current_frame) {
                //     current_frame = range.start;
                // }
                Some(current_frame)
            }
            None => Some(*range.start()),
        };
        debug!("current_frame: {:?}", spritesheet_animation.current_frame);
        texture_atlas_sprite.index = spritesheet_animation.current_frame.unwrap();
        if !spritesheet_animation.end_frame || spritesheet_animation.do_loop {
            spritesheet_animation.frame_timer = Some(Timer::new(
                Duration::from_millis(
                    spritesheet.durations[spritesheet_animation.current_frame.unwrap()],
                ),
                false,
            ));
        }
    }
}

#[derive(Default)]
struct SpritesheetLoader {}

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

#[derive(Default)]
pub struct SpritesheetPlugin;

impl Plugin for SpritesheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Spritesheet>()
            .init_asset_loader::<SpritesheetLoader>()
            .add_system(spritesheet_animation_system);
    }
}
