use bevy::prelude::*;

/// Configuration for the dialogue view that users can customize
#[derive(Debug, Clone, Resource)]
pub struct DialogueViewConfig {
    /// Text direction for the dialogue
    pub text_direction: TextDirection,
    /// Size of the dialogue box
    pub dialogue_size: DialogueSize,
    /// 3D position for the dialogue (if using 3D positioning)
    pub position_3d: Option<Vec3>,
    /// Whether to use 3D positioning instead of screen positioning
    pub use_3d_positioning: bool,
    /// Background color of the dialogue box
    pub background_color: Color,
    /// Border radius of the dialogue box
    pub border_radius: f32,
    /// Padding around the text
    pub padding: UiRect,
}

impl Default for DialogueViewConfig {
    fn default() -> Self {
        Self {
            text_direction: TextDirection::LeftToRight,
            dialogue_size: DialogueSize::default(),
            position_3d: None,
            use_3d_positioning: false,
            background_color: Color::BLACK.with_alpha(0.8),
            border_radius: 20.0,
            padding: UiRect::all(Val::Px(20.0)),
        }
    }
}

/// Text direction options for dialogue
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    /// Left to right (default for most languages)
    LeftToRight,
    /// Right to left (for languages like Arabic, Hebrew)
    RightToLeft,
    /// Top to bottom (for vertical text)
    TopToBottom,
    /// Bottom to top (for vertical text)
    BottomToTop,
}

/// Dialogue size configuration
#[derive(Debug, Clone)]
pub struct DialogueSize {
    /// Width of the dialogue box
    pub width: Val,
    /// Height of the dialogue box
    pub height: Val,
    /// Maximum width (for responsive design)
    pub max_width: Val,
    /// Maximum height (for responsive design)
    pub max_height: Val,
}

impl Default for DialogueSize {
    fn default() -> Self {
        Self {
            width: Val::Px(800.0 * 0.8),
            height: Val::Auto,
            max_width: Val::Px(800.0 * 0.8),
            max_height: Val::Px(200.0),
        }
    }
}

impl DialogueSize {
    /// Create a fixed size dialogue
    pub fn fixed(width: f32, height: f32) -> Self {
        Self {
            width: Val::Px(width),
            height: Val::Px(height),
            max_width: Val::Px(width),
            max_height: Val::Px(height),
        }
    }

    /// Create a responsive dialogue that adapts to content
    pub fn responsive(max_width: f32, max_height: f32) -> Self {
        Self {
            width: Val::Auto,
            height: Val::Auto,
            max_width: Val::Px(max_width),
            max_height: Val::Px(max_height),
        }
    }

    /// Create a percentage-based dialogue
    pub fn percentage(width_percent: f32, height_percent: f32) -> Self {
        Self {
            width: Val::Percent(width_percent),
            height: Val::Percent(height_percent),
            max_width: Val::Percent(width_percent),
            max_height: Val::Percent(height_percent),
        }
    }
}

/// Builder pattern for easy configuration
impl DialogueViewConfig {
    /// Set the text direction
    pub fn with_text_direction(mut self, direction: TextDirection) -> Self {
        self.text_direction = direction;
        self
    }

    /// Set the dialogue size
    pub fn with_size(mut self, size: DialogueSize) -> Self {
        self.dialogue_size = size;
        self
    }

    /// Set a fixed 3D position
    pub fn with_3d_position(mut self, position: Vec3) -> Self {
        self.position_3d = Some(position);
        self.use_3d_positioning = true;
        self
    }

    /// Enable 3D positioning
    pub fn with_3d_positioning(mut self, enabled: bool) -> Self {
        self.use_3d_positioning = enabled;
        self
    }

    /// Set the background color
    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Set the border radius
    pub fn with_border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Set the padding
    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }
}
