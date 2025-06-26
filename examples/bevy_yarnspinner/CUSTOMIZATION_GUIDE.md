# Bevy YarnSpinner Dialogue View Customization Guide

This guide shows you how to customize the Bevy YarnSpinner dialogue view with advanced features including text alignment, text direction, dialogue size, and 3D positioning.

## Quick Start

```rust
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::{
    DialogueViewConfig, DialogueSize, TextDirection, TextAlignment,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YarnSpinnerPlugin::new())
        .add_plugins(ExampleYarnSpinnerDialogueViewPlugin::new())
        .insert_resource(create_custom_config())
        .run();
}

fn create_custom_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_alignment(TextAlignment::Center)
        .with_text_direction(TextDirection::LeftToRight)
        .with_size(DialogueSize::fixed(800.0, 200.0))
        .with_3d_position(Vec3::new(0.0, 2.0, 5.0))
        .with_background_color(Color::rgb(0.1, 0.1, 0.3).with_alpha(0.9))
        .with_border_radius(20.0)
        .with_padding(UiRect::all(Val::Px(25.0)))
}
```

## 1. Text Alignment

Control how text is aligned within the dialogue box:

```rust
use bevy_yarnspinner_example_dialogue_view::{DialogueViewConfig, TextAlignment};

// Left-aligned text (default)
fn create_left_aligned_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_alignment(TextAlignment::Left)
}

// Center-aligned text - perfect for important dialogue or titles
fn create_centered_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_alignment(TextAlignment::Center)
}

// Right-aligned text
fn create_right_aligned_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_alignment(TextAlignment::Right)
}

// Justified text (spreads text across the full width)
fn create_justified_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_alignment(TextAlignment::Justified)
}
```

### Available Text Alignments:
- `TextAlignment::Left` - Left-aligned text (default)
- `TextAlignment::Center` - Center-aligned text
- `TextAlignment::Right` - Right-aligned text  
- `TextAlignment::Justified` - Justified text (spreads across full width)

## 2. Text Direction Customization

You can customize the text direction for different languages and layouts:

```rust
use bevy_yarnspinner_example_dialogue_view::{DialogueViewConfig, TextDirection};

// Left to right (default for English)
fn create_ltr_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_direction(TextDirection::LeftToRight)
}

// Right to left (for Arabic, Hebrew)
fn create_rtl_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_direction(TextDirection::RightToLeft)
}

// Vertical text (for Japanese, Chinese)
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

## 3. Dialogue Size Customization

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

## 4. 3D Positioning

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

## 5. Visual Customization

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

## 6. Complete Example

Here's a complete example combining all features with centered text:

```rust
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::{
    DialogueViewConfig, DialogueSize, TextDirection, TextAlignment,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(YarnSpinnerPlugin::new())
        .add_plugins(ExampleYarnSpinnerDialogueViewPlugin::new())
        .insert_resource(create_advanced_dialogue_config())
        .add_systems(Startup, setup)
        .run();
}

fn create_advanced_dialogue_config() -> DialogueViewConfig {
    DialogueViewConfig::default()
        .with_text_alignment(TextAlignment::Center) // Center the text
        .with_text_direction(TextDirection::LeftToRight)
        .with_size(DialogueSize::fixed(700.0, 180.0))
        .with_3d_position(Vec3::new(0.0, 1.5, 0.0))
        .with_background_color(Color::rgb(0.1, 0.1, 0.3).with_alpha(0.9))
        .with_border_radius(15.0)
        .with_padding(UiRect::all(Val::Px(25.0)))
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Load and start the dialogue
    let project = YarnProject::new()
        .with_file(asset_server.load("dialogue/hello_world.yarn"))
        .build()
        .unwrap();

    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner.start_node("Start");

    commands.spawn(dialogue_runner);
}
```

## 7. Dynamic Configuration

You can also change the configuration at runtime:

```rust
fn update_dialogue_config(
    mut config: ResMut<DialogueViewConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Key1) {
        // Switch to centered text
        config.text_alignment = TextAlignment::Center;
    } else if keyboard.just_pressed(KeyCode::Key2) {
        // Switch to left-aligned text
        config.text_alignment = TextAlignment::Left;
    } else if keyboard.just_pressed(KeyCode::Key3) {
        // Switch to RTL
        config.text_direction = TextDirection::RightToLeft;
    } else if keyboard.just_pressed(KeyCode::Key4) {
        // Switch back to LTR
        config.text_direction = TextDirection::LeftToRight;
    }
}
```

## 8. Multiple Dialogue Views

You can create multiple dialogue views with different configurations:

```rust
fn create_multiple_dialogue_configs() -> Vec<DialogueViewConfig> {
    vec![
        // Main dialogue - centered text
        DialogueViewConfig::default()
            .with_text_alignment(TextAlignment::Center)
            .with_size(DialogueSize::fixed(800.0, 200.0))
            .with_background_color(Color::rgb(0.1, 0.1, 0.3).with_alpha(0.9)),
        
        // Character name - left-aligned
        DialogueViewConfig::default()
            .with_text_alignment(TextAlignment::Left)
            .with_size(DialogueSize::fixed(400.0, 50.0))
            .with_background_color(Color::rgb(0.2, 0.2, 0.2).with_alpha(0.8)),
        
        // Subtitles - right-aligned
        DialogueViewConfig::default()
            .with_text_alignment(TextAlignment::Right)
            .with_size(DialogueSize::responsive(600.0, 100.0))
            .with_background_color(Color::rgb(0.0, 0.0, 0.0).with_alpha(0.7)),
    ]
}
```

## Running the Example

To run the customized dialogue example:

```bash
cargo run --bin customized_dialogue
```

## Tips and Best Practices

- **Text Alignment**: Use `TextAlignment::Center` for important dialogue or titles
- **3D Positioning**: Great for positioning dialogue near characters or objects in 3D space
- **Responsive Design**: Use `DialogueSize::responsive()` for dialogue that adapts to different screen sizes
- **Accessibility**: Consider using appropriate text directions for different languages
- **Performance**: 3D positioning requires additional calculations, so disable it if not needed

## Troubleshooting

- **Text not centering**: Make sure you're using `TextAlignment::Center` and the dialogue box is wide enough
- **3D positioning not working**: Ensure you have a 3D camera and the positioning is enabled
- **Text direction issues**: For RTL languages, you may need additional font support
- **Size not applying**: Check that your size values are reasonable and the dialogue box has enough space
