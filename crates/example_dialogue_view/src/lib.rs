//! A simple example dialogue view for Yarn Spinner.
//! A dialogue view is a plugin that handles presenting lines and options to the user and advances the dialogue on user input.
//! This one shows text in a dialogue box inspired by Legend of Zelda: Breath of the Wild.
//!
//! ## Demo
//!
//! The [Yarn Spinner for Rust Demo](https://janhohenheim.itch.io/yarnspinner-rust-demo) uses this dialogue view, so you can play that in the browser if you
//! want to see it in action. Additionally, all [Bevy Yarn Spinner examples](https://github.com/YarnSpinnerTool/YarnSpinner-Rust/tree/main/examples/bevy_yarnspinner/src/bin) use
//! this dialogue view as well.
//!
//! ## Usage
//!
//! It's enough to simply register [`ExampleYarnSpinnerDialogueViewPlugin`] alongside [`YarnSpinnerPlugin`]:
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_yarnspinner::*;
//! use bevy_yarnspinner::prelude::YarnSpinnerPlugin;
//! use bevy_yarnspinner_example_dialogue_view::prelude::*;
//!
//! App::new()
//!    .add_plugins(DefaultPlugins)
//!    .add_plugins(YarnSpinnerPlugin::new())
//!    .add_plugins(ExampleYarnSpinnerDialogueViewPlugin::new());
//! ```
//!
//! ## Customization
//!
//! You can customize the dialogue view with various options:
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_yarnspinner::prelude::*;
//! use bevy_yarnspinner_example_dialogue_view::prelude::*;
//!
//! App::new()
//!    .add_plugins(DefaultPlugins)
//!    .add_plugins(YarnSpinnerPlugin::new())
//!    .insert_resource(DialogueViewConfig::default()
//!        .with_text_alignment(TextAlignment::Center)
//!        .with_text_direction(TextDirection::RightToLeft)
//!        .with_size(DialogueSize::fixed(600.0, 150.0))
//!        .with_3d_position(Vec3::new(0.0, 2.0, 0.0))
//!        .with_background_color(Color::BLUE.with_alpha(0.9))
//!    )
//!    .add_plugins(ExampleYarnSpinnerDialogueViewPlugin::new());
//! ```
//!
//! This crate also exposes the [`SpeakerChangeEvent`] which you can use to animate characters while they are speaking,
//! as the text is written out over a few seconds.
//!
//! ## Inputs
//!
//! - Advance the dialogue: press the space bar, enter key, left click or tap the screen after the text is done typing.
//! - Type out the text faster: Same as above, but hold press before the text is done typing.
//! - Select an option: press the number key corresponding to the option you want to select or click/tap the option.
//!
//! ## Limitations
//!
//! This dialogue view expects only a single instance of [`DialogueRunner`](bevy_yarnspinner::prelude::DialogueRunner) to be running.
//! Its behavior is otherwise undefined.

#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs, missing_debug_implementations)]

use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;
pub use setup::UiRootNode;
pub use updating::SpeakerChangeEvent;

pub mod prelude {
    //! Everything you need to get starting using this example Yarn Spinner dialogue view.
    pub use crate::{
        ExampleYarnSpinnerDialogueViewPlugin, ExampleYarnSpinnerDialogueViewSystemSet,
        SpeakerChangeEvent, DialogueViewConfig, TextDirection, TextAlignment, DialogueSize,
    };
}

/// The plugin registering all systems of the dialogue view.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ExampleYarnSpinnerDialogueViewPlugin;

/// The [`SystemSet`] containing all systems added by the [`ExampleYarnSpinnerDialogueViewPlugin`].
/// Is run after the [`YarnSpinnerSystemSet`](bevy_yarnspinner::prelude::YarnSpinnerSystemSet).
#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct ExampleYarnSpinnerDialogueViewSystemSet;

impl ExampleYarnSpinnerDialogueViewPlugin {
    /// Creates a new example dialogue view
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new example dialogue view with custom configuration
    pub fn with_config(config: DialogueViewConfig) -> Self {
        // Store the config as a resource
        let mut app = App::new();
        app.insert_resource(config);
        Self::default()
    }
}

mod assets;
mod config;
mod option_selection;
mod positioning;
mod setup;
mod typewriter;
mod updating;

impl Plugin for ExampleYarnSpinnerDialogueViewPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<YarnSpinnerPlugin>(),
            "YarnSpinnerPlugin must be added before ExampleYarnSpinnerDialogueViewPlugin"
        );

        app.add_plugins(assets::ui_assets_plugin)
            .add_plugins(setup::ui_setup_plugin)
            .add_plugins(updating::ui_updating_plugin)
            .add_plugins(typewriter::typewriter_plugin)
            .add_plugins(option_selection::option_selection_plugin)
            .add_systems(Update, positioning::position_dialogue_3d);
    }
}

// Re-export configuration types for easy access
pub use config::{DialogueViewConfig, TextDirection, TextAlignment, DialogueSize};
pub use positioning::Dialogue3DPosition;

#[cfg(doctest)]
#[doc = include_str!("../../../readme.md")]
mod test_readme {}
