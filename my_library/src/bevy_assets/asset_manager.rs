use crate::{AssetStore, FutureAtlas};
use bevy::app::{Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::ecs::system::{Commands, Res, Resource};
use bevy::math::Vec2;

#[derive(Clone)]
pub enum AssetType {
    Image,
    Sound,
    SpriteSheet {
        tile_size: Vec2,
        sprites_x: usize,
        sprites_y: usize,
    },
}

#[derive(Resource, Clone)]
pub struct AssetManager {
    asset_list: Vec<(String, String, AssetType)>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            asset_list: vec![
                (
                    "main_menu".to_string(),
                    "main_menu.png".to_string(),
                    AssetType::Image,
                ),
                (
                    "game_over".to_string(),
                    "game_over.png".to_string(),
                    AssetType::Image,
                ),
            ],
        }
    }

    pub fn add_image<S: ToString>(mut self, tag: S, filename: S) -> anyhow::Result<Self> {
        let filename = filename.to_string();
        AssetManager::asset_exists(&filename)?;
        self.asset_list
            .push((tag.to_string(), filename, AssetType::Image));
        Ok(self)
    }

    pub fn add_sound<S: ToString>(mut self, tag: S, filename: S) -> anyhow::Result<Self> {
        let filename = filename.to_string();
        AssetManager::asset_exists(&filename)?;
        self.asset_list
            .push((tag.to_string(), filename, AssetType::Sound));
        Ok(self)
    }

    pub fn add_sprite_sheet<S: ToString>(
        mut self,
        tag: S,
        filename: S,
        sprite_width: f32,
        sprite_height: f32,
        sprites_x: usize,
        sprites_y: usize,
    ) -> anyhow::Result<Self> {
        let filename = filename.to_string();
        AssetManager::asset_exists(&filename)?;
        self.asset_list.push((
            tag.to_string(),
            filename,
            AssetType::SpriteSheet {
                tile_size: Vec2::new(sprite_width, sprite_height),
                sprites_x,
                sprites_y,
            },
        ));
        Ok(self)
    }

    fn asset_exists(filename: &str) -> anyhow::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let current_directory = std::env::current_dir()?;
            let assets = current_directory.join("assets");
            let new_image = assets.join(filename);
            if !new_image.exists() {
                return Err(anyhow::Error::msg(format!(
                    "{} not found in assets directory",
                    &filename
                )));
            }
        }
        Ok(())
    }
}

impl Plugin for AssetManager {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.clone());
        app.add_systems(Startup, setup);
    }
}

fn setup(
    asset_resource: Res<AssetManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut assets = AssetStore {
        asset_index: bevy::utils::HashMap::new(),
        atlases_to_build: Vec::new(),
        atlases: bevy::utils::HashMap::new(),
    };
    asset_resource
        .asset_list
        .iter()
        .for_each(|(tag, filename, asset_type)| {
            match asset_type {
                AssetType::SpriteSheet {
                    tile_size,
                    sprites_x,
                    sprites_y,
                } => {
                    //Sprite Sheets require that we load the image first, and defer
                    //sheet creation to the loading menu - after the image has loaded.
                    let image_handle = asset_server.load_untyped(filename);
                    let base_tag = format!("{tag}_base");
                    assets.asset_index.insert(base_tag.clone(), image_handle);

                    //Now that its loaded, we store the future atlas in the asset store.
                    assets.atlases_to_build.push(FutureAtlas {
                        tag: tag.clone(),
                        texture_tag: base_tag,
                        tile_size: *tile_size,
                        sprites_x: *sprites_x,
                        sprites_y: *sprites_y,
                    });
                }
                _ => {
                    //Most asset types don't require a separate loaded
                    assets
                        .asset_index
                        .insert(tag.clone(), asset_server.load_untyped(filename));
                }
            }
        });
    commands.remove_resource::<AssetManager>();
    commands.insert_resource(assets);
}
