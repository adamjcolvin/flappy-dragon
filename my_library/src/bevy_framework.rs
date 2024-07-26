mod bevy_animation;
use bevy::prelude::*;
pub use bevy_animation::*;
mod bevy_physics;
mod game_menus;
pub use bevy_physics::*;

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
        app.add_event::<PhysicsTick>();
        app.add_event::<Impulse>();
        app.add_state::<T>(); //(2)
        app.add_plugins(bevy_egui::EguiPlugin);
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

        app.add_systems(OnEnter(T::default()), crate::bevy_assets::setup);
        app.add_systems(
            Update,
            crate::bevy_assets::run::<T>.run_if(in_state(T::default())),
        );
        app.add_systems(OnExit(T::default()), crate::bevy_assets::exit);
    }
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
