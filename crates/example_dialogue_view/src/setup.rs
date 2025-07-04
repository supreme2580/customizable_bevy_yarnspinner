use crate::assets::image_handle;
use crate::config::{DialogueViewConfig, TextDirection, TextAlignment};
use crate::positioning::Dialogue3DPosition;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

pub(crate) fn ui_setup_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

/// Marker for the [`Node`] that is the root of the UI
#[derive(Debug, Default, Component)]
pub struct UiRootNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNameNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueContinueNode;

#[derive(Debug, Default, Component)]
pub(crate) struct OptionsNode;

#[derive(Debug, Component)]
pub(crate) struct OptionButton(pub OptionId);

fn setup(mut commands: Commands, config: Option<Res<DialogueViewConfig>>) {
    let config = if let Some(config) = config {
        config.clone()
    } else {
        DialogueViewConfig::default()
    };

    // root node
    let mut root_entity = commands.spawn((
        fmt_name("root"),
        Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_content: AlignContent::End,
            justify_content: JustifyContent::SpaceAround,
            grid_auto_flow: GridAutoFlow::Row,
            grid_template_columns: vec![RepeatedGridTrack::minmax(
                1,
                MinTrackSizingFunction::Auto,
                MaxTrackSizingFunction::Px(get_width_value(&config.dialogue_size.width)),
            )],
            ..default()
        },
        Visibility::Hidden,
        UiRootNode,
    ));

    // Add 3D position component if enabled
    if config.use_3d_positioning {
        if let Some(position) = config.position_3d {
            root_entity.insert(Dialogue3DPosition {
                world_position: position,
                offset: Vec2::ZERO,
            });
        }
    }

    root_entity.with_children(|parent| {
        parent.spawn((
            fmt_name("name"),
            Text::default(),
            Node {
                margin: UiRect {
                    left: Val::Px(TEXT_BORDER_HORIZONTAL / 2.0),
                    bottom: Val::Px(-8.0),
                    ..default()
                },
                ..default()
            },
            ZIndex(1),
            DialogueNameNode,
            Label,
        ));

        parent
            .spawn((
                fmt_name("dialogue"),
                Node {
                    flex_direction: get_flex_direction(config.text_direction),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: get_align_items(config.text_direction),
                    width: config.dialogue_size.width,
                    height: config.dialogue_size.height,
                    max_width: config.dialogue_size.max_width,
                    max_height: config.dialogue_size.max_height,
                    padding: config.padding,
                    ..default()
                },
                BackgroundColor(config.background_color),
                BorderRadius::all(Val::Px(config.border_radius)),
            ))
            .with_children(|parent| {
                // Dialog itself
                parent.spawn((
                    fmt_name("text"),
                    Text::default(),
                    Node {
                        justify_content: get_text_justify_content(config.text_alignment),
                        align_items: get_text_align_items(config.text_alignment),
                        max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER_HORIZONTAL),
                        ..default()
                    },
                    DialogueNode,
                    Label,
                ));
            })
            .with_children(|parent| {
                // Options
                parent.spawn((
                    fmt_name("options"),
                    Node {
                        display: Display::None,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexStart,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    Visibility::Hidden,
                    OptionsNode,
                ));
            });

        parent.spawn((
            fmt_name("continue indicator"),
            ImageNode {
                // 27 x 27 pixels
                image: image_handle::CONTINUE_INDICATOR,
                ..default()
            },
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                margin: UiRect {
                    top: Val::Px(-18.),
                    bottom: Val::Px(25.),
                    ..default()
                },
                ..default()
            },
            ZIndex(1),
            Visibility::Hidden,
            DialogueContinueNode,
        ));
    });
}

/// Helper function to extract width value from Val
fn get_width_value(val: &Val) -> f32 {
    match val {
        Val::Px(px) => *px,
        Val::Percent(percent) => 800.0 * (percent / 100.0), // Default fallback
        _ => 800.0 * 0.8, // Default fallback
    }
}

/// Get the appropriate flex direction based on text direction
fn get_flex_direction(text_direction: TextDirection) -> FlexDirection {
    match text_direction {
        TextDirection::LeftToRight | TextDirection::RightToLeft => FlexDirection::Column,
        TextDirection::TopToBottom | TextDirection::BottomToTop => FlexDirection::Row,
    }
}

/// Get the appropriate align items based on text direction
fn get_align_items(text_direction: TextDirection) -> AlignItems {
    match text_direction {
        TextDirection::LeftToRight => AlignItems::FlexStart,
        TextDirection::RightToLeft => AlignItems::FlexEnd,
        TextDirection::TopToBottom => AlignItems::FlexStart,
        TextDirection::BottomToTop => AlignItems::FlexEnd,
    }
}

/// Get the appropriate justify content based on text alignment
fn get_text_justify_content(text_alignment: TextAlignment) -> JustifyContent {
    match text_alignment {
        TextAlignment::Left => JustifyContent::FlexStart,
        TextAlignment::Center => JustifyContent::Center,
        TextAlignment::Right => JustifyContent::FlexEnd,
        TextAlignment::Justified => JustifyContent::SpaceBetween,
    }
}

/// Get the appropriate align items based on text alignment
fn get_text_align_items(text_alignment: TextAlignment) -> AlignItems {
    match text_alignment {
        TextAlignment::Left => AlignItems::FlexStart,
        TextAlignment::Center => AlignItems::Center,
        TextAlignment::Right => AlignItems::FlexEnd,
        TextAlignment::Justified => AlignItems::Stretch,
    }
}

fn fmt_name(name: &str) -> Name {
    Name::new(format!("Yarn Spinner example dialogue view node: {name}"))
}

pub(crate) fn spawn_options<'a, T>(entity_commands: &mut EntityCommands, options: T)
where
    T: IntoIterator<Item = &'a DialogueOption>,
    <T as IntoIterator>::IntoIter: 'a,
{
    entity_commands.with_children(|parent| {
        for (i, option) in options.into_iter().enumerate() {
            parent
                .spawn((
                    fmt_name("option button"),
                    Node {
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            fmt_name("option text"),
                            Button,
                            Text::default(),
                            ImageNode::default().with_color(Color::NONE),
                            OptionButton(option.id),
                            Label,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextSpan(format!("{}: ", i + 1)));
                            parent.spawn((
                                TextSpan(option.line.text.clone()),
                            ));
                        });
                });
        }
    });
}

const DIALOG_WIDTH: f32 = 800.0 * 0.8;
const TEXT_BORDER_HORIZONTAL: f32 = 120.0;
