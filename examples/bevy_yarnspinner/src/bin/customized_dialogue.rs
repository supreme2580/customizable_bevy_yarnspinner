use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::{
    DialogueViewConfig, DialogueSize, TextDirection, TextAlignment, ExampleYarnSpinnerDialogueViewPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YarnSpinnerPlugin::new())
        .add_plugins(ExampleYarnSpinnerDialogueViewPlugin::new())
        .insert_resource(create_custom_config())
        .add_systems(Startup, setup_camera)
        .add_systems(Update, spawn_dialogue_runner.run_if(resource_added::<YarnProject>))
        .run();
}

fn create_custom_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        // Center the text
        .with_text_alignment(TextAlignment::Center)
        // Use left-to-right text direction for demonstration
        .with_text_direction(TextDirection::LeftToRight)
        // Make the dialogue box larger
        .with_size(DialogueSize::fixed(900.0, 250.0))
        // Position at a specific 3D point
        .with_3d_position(Vec3::new(0.0, 2.0, 5.0))
        // Use a different background color
        .with_background_color(Color::srgb(0.1, 0.1, 0.3).with_alpha(0.9))
        // Add rounded corners
        .with_border_radius(25.0)
        // Add more padding
        .with_padding(UiRect::all(Val::Px(30.0)))
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("Start");
    commands.spawn(dialogue_runner);
}
