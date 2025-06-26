use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;
use bevy_yarnspinner_example_dialogue_view::{DialogueViewConfig, TextDirection, DialogueSize};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        YarnSpinnerPlugin::new(),
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
    .insert_resource(create_custom_dialogue_config())
    .add_systems(Startup, setup_camera)
    .add_systems(
        Update,
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
    )
    .run();
}

fn create_custom_dialogue_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_direction(TextDirection::LeftToRight) // Change to RightToLeft for RTL languages
        .with_size(DialogueSize::fixed(700.0, 180.0)) // Fixed size dialogue
        .with_3d_position(Vec3::new(0.0, 1.5, 0.0)) // Position in 3D space
        .with_background_color(Color::srgb(0.1, 0.1, 0.3).with_alpha(0.9)) // Custom background
        .with_border_radius(15.0) // Rounded corners
        .with_padding(UiRect::all(Val::Px(25.0))) // Custom padding
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.start_node("Start");
    commands.spawn(dialogue_runner);
}
