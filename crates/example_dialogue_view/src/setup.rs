use crate::assets::{font_handle, image_handle};
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
            text_style::name(),
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
                    text_style::standard(),
                    style::standard(),
                    Node {
                        justify_content: get_text_justify_content(config.text_alignment),
                        align_items: get_text_align_items(config.text_alignment),
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

pub(crate) fn create_dialog_text(
    text: impl Into<String>,
    invisible: impl Into<String>,
) -> [(TextSpan, TextFont, TextColor); 2] {
    [
        (
            TextSpan(text.into()),
            text_style::standard().0,
            text_style::standard().1,
        ),
        (
            TextSpan(invisible.into()),
            text_style::standard().0,
            TextColor(Color::NONE),
        ),
    ]
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
                            style::options(),
                            ImageNode::default().with_color(Color::NONE),
                            OptionButton(option.id),
                            Label,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((TextSpan(format!("{}: ", i + 1)), text_style::option_id()));
                            parent.spawn((
                                TextSpan(option.line.text.clone()),
                                text_style::option_text(),
                            ));
                        });
                });
        }
    });
}

const DIALOG_WIDTH: f32 = 800.0 * 0.8;
const TEXT_BORDER_HORIZONTAL: f32 = 120.0;
const TEXT_BORDER_TOP: f32 = 30.0;
const TEXT_BORDER_BOTTOM: f32 = TEXT_BORDER_TOP + 10.0;

mod style {
    use super::*;
    pub(crate) fn standard() -> Node {
        Node {
            max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER_HORIZONTAL),
            ..default()
        }
    }
    pub(crate) fn options() -> Node {
        const INDENT_MODIFIER: f32 = 1.0;
        Node {
            margin: UiRect::horizontal(Val::Px((INDENT_MODIFIER - 1.0) * TEXT_BORDER_HORIZONTAL)),
            max_width: Val::Px(DIALOG_WIDTH - 2.0 * INDENT_MODIFIER * TEXT_BORDER_HORIZONTAL),
            ..default()
        }
    }
}

mod text_style {
    use super::*;
    use bevy::color::palettes::css;
    pub(crate) fn standard() -> (TextFont, TextColor) {
        (
            TextFont {
                font: font_handle::MEDIUM,
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )
    }
    pub(crate) fn name() -> (TextFont, TextColor) {
        (
            TextFont {
                font: font_handle::MEDIUM,
                font_size: 18.0,
                ..standard().0
            },
            standard().1,
        )
    }

    pub(crate) fn option_id() -> (TextFont, TextColor) {
        (
            TextFont {
                font: font_handle::MEDIUM,
                ..option_text().0
            },
            TextColor(css::ALICE_BLUE.into()),
        )
    }

    pub(crate) fn option_text() -> (TextFont, TextColor) {
        (
            TextFont {
                font_size: 18.0,
                ..standard().0
            },
            TextColor(css::TOMATO.into()),
        )
    }
}
