mod asset_manager;
pub use asset_manager::AssetManager;

mod asset_store;
pub use asset_store::*;

#[macro_export]
macro_rules! spawn_image {
    ($assets:expr, $commands:expr, $index:expr, $x:expr, $y:expr, $z:expr, $resource:expr, $($component:expr),*) => {
      $commands.spawn(SpriteBundle {
        texture: $assets.get_handle($index, $resource).unwrap(),
        transform: Transform::from_xyz($x, $y, $z),
        ..default()
      })
      $(
      .insert($component)
      )*
    };
}
