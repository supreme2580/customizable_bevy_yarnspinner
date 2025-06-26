# Bevy YarnSpinner Dialogue View Customization Guide

This guide shows you how to customize the Bevy YarnSpinner dialogue view with advanced features including text direction, dialogue size, and 3D positioning.

## Basic Setup

First, ensure you have the basic setup:

```rust
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YarnSpinnerPlugin::new())
        .add_plugins(ExampleYarnSpinnerDialogueViewPlugin::new())
        .add_systems(Startup, setup_camera)
        .add_systems(Update, spawn_dialogue_runner.run_if(resource_added::<YarnProject>))
        .run();
}
```

## 1. Text Direction Customization

You can customize the text direction for different languages and layouts:

```rust
use bevy_yarnspinner_example_dialogue_view::{DialogueViewConfig, TextDirection};

fn create_dialogue_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_direction(TextDirection::LeftToRight) // Default for English
}

// For Right-to-Left languages (Arabic, Hebrew)
fn create_rtl_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_direction(TextDirection::RightToLeft)
}

// For vertical text (Japanese, Chinese)
fn create_vertical_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_direction(TextDirection::TopToBottom)
}
```

### Available Text Directions:
- `TextDirection::LeftToRight` - Standard left-to-right text (English, French, etc.)
- `TextDirection::RightToLeft` - Right-to-left text (Arabic, Hebrew)
- `TextDirection::TopToBottom` - Vertical text top-to-bottom (Japanese, Chinese)
- `TextDirection::BottomToTop` - Vertical text bottom-to-top

## 2. Dialogue Size Customization

You can easily customize the size of your dialogue boxes:

```rust
use bevy_yarnspinner_example_dialogue_view::{DialogueViewConfig, DialogueSize};

// Fixed size dialogue
fn create_fixed_size_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_size(DialogueSize::fixed(600.0, 150.0))
}

// Responsive dialogue that adapts to content
fn create_responsive_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_size(DialogueSize::responsive(800.0, 200.0))
}

// Percentage-based dialogue
fn create_percentage_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_size(DialogueSize::percentage(80.0, 30.0)) // 80% width, 30% height
}
```

### Size Options:
- `DialogueSize::fixed(width, height)` - Fixed pixel dimensions
- `DialogueSize::responsive(max_width, max_height)` - Adapts to content with limits
- `DialogueSize::percentage(width_percent, height_percent)` - Percentage of screen size

## 3. 3D Positioning

You can position dialogue boxes at specific 3D world coordinates:

```rust
use bevy_yarnspinner_example_dialogue_view::DialogueViewConfig;

// Position dialogue at a specific 3D point
fn create_3d_positioned_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_3d_position(Vec3::new(0.0, 2.0, 0.0)) // Above the origin
}

// Position dialogue above a character
fn create_character_dialogue_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_3d_position(Vec3::new(1.5, 1.8, 0.0)) // Above character position
}
```

### 3D Positioning Features:
- Dialogue boxes automatically follow the camera
- Position is converted from 3D world space to screen space
- Works with both 2D and 3D cameras
- Automatically updates when camera moves

## 4. Visual Customization

You can customize the appearance of your dialogue boxes:

```rust
fn create_custom_visual_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_background_color(Color::rgb(0.1, 0.1, 0.3).with_alpha(0.9)) // Dark blue background
        .with_border_radius(15.0) // Rounded corners
        .with_padding(UiRect::all(Val::Px(25.0))) // Custom padding
}
```

### Visual Options:
- `with_background_color(color)` - Set background color and transparency
- `with_border_radius(radius)` - Set corner roundness
- `with_padding(padding)` - Set internal spacing

## 5. Complete Example

Here's a complete example combining all features:

```rust
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YarnSpinnerPlugin::new())
        .add_plugins(ExampleYarnSpinnerDialogueViewPlugin::new())
        .insert_resource(create_advanced_dialogue_config())
        .add_systems(Startup, setup_camera)
        .add_systems(Update, spawn_dialogue_runner.run_if(resource_added::<YarnProject>))
        .run();
}

fn create_advanced_dialogue_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_direction(TextDirection::LeftToRight)
        .with_size(DialogueSize::fixed(700.0, 180.0))
        .with_3d_position(Vec3::new(0.0, 1.5, 0.0))
        .with_background_color(Color::rgb(0.1, 0.1, 0.3).with_alpha(0.9))
        .with_border_radius(15.0)
        .with_padding(UiRect::all(Val::Px(25.0)))
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.start_node("Start");
    commands.spawn(dialogue_runner);
}
```

## 6. Dynamic Configuration

You can also change the configuration at runtime:

```rust
fn update_dialogue_config(
    mut config: ResMut<DialogueViewConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Key1) {
        // Switch to RTL
        config.text_direction = TextDirection::RightToLeft;
    } else if keyboard.just_pressed(KeyCode::Key2) {
        // Switch to vertical text
        config.text_direction = TextDirection::TopToBottom;
    } else if keyboard.just_pressed(KeyCode::Key3) {
        // Switch back to LTR
        config.text_direction = TextDirection::LeftToRight;
    }
}
```

## 7. Multiple Dialogue Views

You can create multiple dialogue views with different configurations:

```rust
fn create_multiple_dialogue_configs() -> Vec<DialogueViewConfig> {
    vec![
        // Main character dialogue
        DialogueViewConfig::default()
            .with_text_direction(TextDirection::LeftToRight)
            .with_size(DialogueSize::fixed(600.0, 150.0))
            .with_3d_position(Vec3::new(0.0, 1.5, 0.0))
            .with_background_color(Color::BLUE.with_alpha(0.8)),
        
        // NPC dialogue
        DialogueViewConfig::default()
            .with_text_direction(TextDirection::RightToLeft)
            .with_size(DialogueSize::fixed(500.0, 120.0))
            .with_3d_position(Vec3::new(2.0, 1.8, 0.0))
            .with_background_color(Color::GREEN.with_alpha(0.8)),
    ]
}
```

## Tips and Best Practices

1. **Text Direction**: Choose the appropriate text direction for your target language
2. **Size**: Use responsive sizing for dynamic content, fixed sizing for consistent layouts
3. **3D Positioning**: Position dialogue boxes slightly above characters for better readability
4. **Colors**: Use semi-transparent backgrounds to maintain visibility of the game world
5. **Performance**: 3D positioning has minimal performance impact and works well with camera movement

## Troubleshooting

- **Text not appearing**: Ensure your Yarn file has the correct node name
- **3D positioning not working**: Make sure you have a camera in your scene
- **Text direction not changing**: The change will be visible on the next dialogue line
- **Size not updating**: The size change will be applied when the dialogue view is recreated 
