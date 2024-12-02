use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) { 
        app.add_plugins(DefaultPlugins.set(WindowPlugin
            {
            primary_window: Some(Window 
                {
                title: "Rust Game".to_string(),
                resolution: (800., 500.).into(),
                resizable: false,
                ..default()
                }),
             ..default()
            }));
    }
}