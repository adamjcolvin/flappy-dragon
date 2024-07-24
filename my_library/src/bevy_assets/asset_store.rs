use bevy::{
    asset::{Asset, LoadedUntypedAsset},
    prelude::*,
    utils::HashMap,
};

pub type LoadedAssets = Assets<LoadedUntypedAsset>;
pub type AssetResource<'w> = Res<'w, LoadedAssets>;

pub(crate) struct FutureAtlas {
    pub(crate) tag: String,
    pub(crate) texture_tag: String,
    pub(crate) tile_size: Vec2,
    pub(crate) sprites_x: usize,
    pub(crate) sprites_y: usize,
}

#[derive(Resource)]
pub struct AssetStore {
    pub(crate) asset_index: HashMap<String, Handle<LoadedUntypedAsset>>,
    pub(crate) atlases_to_build: Vec<FutureAtlas>,
    pub(crate) atlases: HashMap<String, Handle<TextureAtlas>>,
}

impl AssetStore {
    pub fn get_handle<T>(&self, index: &str, assets: &LoadedAssets) -> Option<Handle<T>>
    where
        T: Asset,
    {
        if let Some(handle_untyped) = self.asset_index.get(index) {
            if let Some(handle) = assets.get(handle_untyped) {
                return Some(handle.handle.clone().typed::<T>());
            }
            None
        } else {
            None
        }
    }

    pub fn play(&self, sound_name: &str, commands: &mut Commands, assets: &LoadedAssets) {
        let sound_handle: Handle<AudioSource> = self.get_handle(sound_name, assets).unwrap();
        commands.spawn(AudioBundle {
            source: sound_handle,
            ..default()
        });
    }

    pub fn get_atlas_handle(&self, index: &str) -> Option<Handle<TextureAtlas>> {
        if let Some(handle) = self.atlases.get(index) {
            return Some(handle.clone());
        }
        None
    }
}
