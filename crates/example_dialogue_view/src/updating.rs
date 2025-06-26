use crate::option_selection::OptionSelection;
use crate::setup::{DialogueContinueNode, DialogueNameNode, DialogueNode, UiRootNode};
use crate::typewriter::Typewriter;
use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            hide_dialog,
            show_dialog,
            present_line,
            present_options,
            continue_dialogue,
        )
            .chain()
            .after(YarnSpinnerSystemSet),
    )
    .add_event::<SpeakerChangeEvent>()
    .register_type::<SpeakerChangeEvent>();
}

/// Signals that a speaker has changed.
/// A speaker starts speaking when a new line is presented with a [`PresentLineEvent`] which has a character name.
/// A speaker stops speaking when the line is fully displayed on the screen, which happens over the course of a few seconds
#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
#[reflect(Debug, PartialEq, Hash)]
#[non_exhaustive]
pub struct SpeakerChangeEvent {
    /// The name of the character who is or was speaking.
    pub character_name: String,
    /// If `true`, the character just started speaking. Otherwise, they just stopped.
    pub speaking: bool,
}

fn show_dialog(
    mut line_events: EventReader<DialogueStartEvent>,
    mut visibility: Single<&mut Visibility, With<UiRootNode>>,
) {
    for _event in line_events.read() {
        **visibility = Visibility::Inherited;
    }
}

fn hide_dialog(
    mut root_visibility: Single<&mut Visibility, With<UiRootNode>>,
    mut dialogue_complete_events: EventReader<DialogueCompleteEvent>,
) {
    if !dialogue_complete_events.is_empty() {
        **root_visibility = Visibility::Hidden;
        dialogue_complete_events.clear();
    }
}

fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut commands: Commands,
    name_node: Single<Entity, With<DialogueNameNode>>,
    dialogue_node: Single<Entity, With<DialogueNode>>,
    mut text_writer: TextUiWriter,
) {
    for event in line_events.read() {
        let name = if let Some(name) = event.line.character_name() {
            speaker_change_events.write(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
            name.to_string()
        } else {
            String::new()
        };
        *text_writer.text(*name_node, 0) = name;

        // Create a new typewriter component for this line
        let typewriter = Typewriter::new(event.line.text_without_character_name().to_string());
        commands.entity(*dialogue_node).insert(typewriter);
    }
}

fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.read() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}

fn continue_dialogue(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut typewriter_query: Query<&mut Typewriter, With<DialogueNode>>,
    option_selection: Option<Res<OptionSelection>>,
    mut root_visibility: Single<&mut Visibility, With<UiRootNode>>,
    mut continue_visibility: Single<
        &mut Visibility,
        (With<DialogueContinueNode>, Without<UiRootNode>),
    >,
) {
    let explicit_continue = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
        || mouse_buttons.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();

    // Check if any typewriter is not finished
    let all_finished = typewriter_query.iter().all(|tw| tw.is_finished());

    if explicit_continue && !all_finished {
        // Complete all typewriters
        for mut typewriter in typewriter_query.iter_mut() {
            typewriter.complete();
        }
        return;
    }

    if explicit_continue && option_selection.is_none() {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
                **root_visibility = Visibility::Hidden;
                **continue_visibility = Visibility::Hidden;
            }
        }
    }
}
