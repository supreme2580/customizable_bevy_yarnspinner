use crate::config::{DialogueViewConfig, TextDirection};
use crate::setup::DialogueNode;
use bevy::prelude::*;

pub(crate) fn typewriter_plugin(app: &mut App) {
    app.add_systems(Update, typewriter);
}

#[derive(Debug, Component)]
pub(crate) struct Typewriter {
    pub(crate) text: String,
    pub(crate) invisible: String,
    pub(crate) timer: Timer,
    pub(crate) current_index: usize,
    pub(crate) is_complete: bool,
}

impl Typewriter {
    pub(crate) fn new(text: String) -> Self {
        Self {
            text,
            invisible: String::new(),
            timer: Timer::from_seconds(0.03, TimerMode::Repeating),
            current_index: 0,
            is_complete: false,
        }
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.is_complete
    }

    pub(crate) fn complete(&mut self) {
        self.invisible = self.text.clone();
        self.current_index = self.text.len();
        self.is_complete = true;
    }
}

fn typewriter(
    mut typewriter_query: Query<(&mut Typewriter, &mut Text, &mut TextColor, &mut TextFont), With<DialogueNode>>,
    config: Option<Res<DialogueViewConfig>>,
    time: Res<Time>,
) {
    let config = if let Some(config) = config {
        config.clone()
    } else {
        DialogueViewConfig::default()
    };

    for (mut typewriter, mut text, mut color, mut font) in typewriter_query.iter_mut() {
        if typewriter.is_complete {
            continue;
        }

        typewriter.timer.tick(time.delta());

        if typewriter.timer.just_finished() {
            if typewriter.current_index < typewriter.text.len() {
                let char = typewriter.text.chars().nth(typewriter.current_index).unwrap();
                typewriter.invisible.push(char);
                typewriter.current_index += 1;
            } else {
                typewriter.is_complete = true;
            }

            // Format text based on direction
            let formatted_invisible = format_text_for_direction(&typewriter.invisible, config.text_direction);

            // Set the text, color, and font size
            text.0 = formatted_invisible;
            color.0 = config.text_color; // Use the color from DialogueViewConfig
            font.font_size = 24.0; // Or your preferred size
            // font.font = my_font_handle.clone().into(); // Set this if you want a custom font
        }
    }
}

/// Format text based on the specified text direction
fn format_text_for_direction(text: &str, direction: TextDirection) -> String {
    match direction {
        TextDirection::LeftToRight => text.to_string(),
        TextDirection::RightToLeft => {
            // For RTL, we need to reverse the text and handle bidirectional text properly
            // This is a simplified implementation - for production use, consider using a proper RTL library
            text.chars().rev().collect()
        }
        TextDirection::TopToBottom => {
            // For vertical text, insert line breaks between characters
            text.chars().map(|c| c.to_string()).collect::<Vec<_>>().join("\n")
        }
        TextDirection::BottomToTop => {
            // For vertical text bottom-to-top, reverse the characters and add line breaks
            text.chars().rev().map(|c| c.to_string()).collect::<Vec<_>>().join("\n")
        }
    }
}
