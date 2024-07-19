use bevy::prelude::*;
mod game_menus;

pub struct GameStatePlugin<T> {
    menu_state: T,
    game_start_state: T,
    game_end_state: T,
}

impl<T> GameStatePlugin<T>
where
    T: States,
{
    #[allow(clippy::new_without_default)]
    pub fn new(menu_state: T, game_start_state: T, game_end_state: T) -> Self {
        Self {
            menu_state,
            game_start_state,
            game_end_state,
        } //(1)
    }
}

impl<T> Plugin for GameStatePlugin<T>
where
    T: States + Copy,
{
    fn build(&self, app: &mut App) {
        app.add_state::<T>(); //(2)
        app.add_systems(Startup, setup_menus); //(3)
        let start = MenuResource {
            menu_state: self.menu_state,
            game_start_state: self.game_start_state, //(4)
            game_end_state: self.game_end_state,
        };
        app.insert_resource(start);

        app.add_systems(OnEnter(self.menu_state), game_menus::setup::<T>);
        app.add_systems(
            Update,
            game_menus::run::<T>.run_if(in_state(self.menu_state)),
        );
        app.add_systems(OnExit(self.menu_state), cleanup::<game_menus::MenuElement>); //(5)

        app.add_systems(OnEnter(self.game_end_state), game_menus::setup::<T>);
        app.add_systems(
            Update,
            game_menus::run::<T>.run_if(in_state(self.game_end_state)),
        );
        app.add_systems(
            OnExit(self.game_end_state),
            cleanup::<game_menus::MenuElement>,
        );
    }
}

#[derive(Resource)]
pub(crate) struct MenuAssets {
    //(6)
    pub(crate) main_menu: Handle<Image>,
    pub(crate) game_over: Handle<Image>,
}

fn setup_menus(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = MenuAssets {
        main_menu: asset_server.load("main_menu.png"),
        game_over: asset_server.load("game_over.png"),
    };
    commands.insert_resource(assets);
}

#[derive(Resource)]
pub(crate) struct MenuResource<T> {
    pub(crate) menu_state: T,
    pub(crate) game_start_state: T,
    pub(crate) game_end_state: T,
}

pub fn cleanup<T>(query: Query<Entity, With<T>>, mut commands: Commands)
where
    T: Component,
{
    query.for_each(|entity| commands.entity(entity).despawn())
}
