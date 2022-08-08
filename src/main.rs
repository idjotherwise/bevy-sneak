mod camera;
mod food;
mod grid;
mod sneak;

mod prelude {
    pub use crate::camera::setup_camera;
    pub use crate::food::*;
    pub use crate::grid::{position_translation, size_scaling, Position, Size};
    pub use crate::sneak::{
        game_over, sneak_eating, sneak_growth, sneak_movement, sneak_movement_input, spawn_sneak,
        GameOverEvent, GrowthEvent, LastTailPosition, SneakSegments,
    };
    pub use bevy::core::FixedTimestep;
    pub use bevy::prelude::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Sneak!".to_string(),
            width: 500.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(SneakSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_sneak)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(food_spawner),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(sneak_movement)
                .with_system(sneak_eating.after(sneak_movement))
                .with_system(sneak_growth.after(sneak_eating)),
        )
        .add_system(game_over.after(sneak_movement))
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_system(sneak_movement_input.before(sneak_movement))
        .add_plugins(DefaultPlugins)
        .run();
}
