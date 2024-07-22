use crate::{egui::egui::Window, AssetStore, MenuResource};
use bevy::{asset::LoadedUntypedAsset, prelude::*};
use bevy_egui::EguiContexts;

#[derive(Resource)]
pub(crate) struct AssetsToLoad(Vec<Handle<LoadedUntypedAsset>>); //(1)

pub(crate) fn setup(assets: Res<AssetStore>, mut commands: Commands) {
    let assets_to_load: Vec<Handle<LoadedUntypedAsset>> =
        assets.asset_index.values().cloned().collect(); //(2)
    commands.insert_resource(AssetsToLoad(assets_to_load)); //(3)
}

pub(crate) fn run<T>(
    asset_server: Res<AssetServer>,
    mut to_load: ResMut<AssetsToLoad>,
    mut state: ResMut<NextState<T>>,
    mut egui_context: EguiContexts,
    menu_info: Res<MenuResource<T>>,
) where
    T: States,
{
    to_load.0.retain(|handle| {
        //(4)
        match asset_server.get_load_state(handle.id()) {
            //(5)
            Some(bevy::asset::LoadState::Loaded) => false, //(6)
            _ => true,                                     //(7)
        }
    });
    if to_load.0.is_empty() {
        //(8)
        state.set(menu_info.menu_state.clone());
    }
    Window::new("Loading, Please Wait").show(
        //(9)
        egui_context.ctx_mut(),
        |ui| ui.label(format!("{} assets remaining", to_load.0.len())),
    );
}

pub(crate) fn exit(mut commands: Commands) {
    commands.remove_resource::<AssetsToLoad>();
}
