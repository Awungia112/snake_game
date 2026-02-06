use ratatui::style::Color;

// Theme Definitions
pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub border: Color,
    pub snake_head: Color,
    pub snake_body: Color,
    pub food: Color,
    pub obstacle: Color,
    pub ghost: Color,
    pub multiplier: Color,
    pub combo: Color,
}

pub const THEMES: &[Theme] = &[
    // Classic
    Theme {
        name: "Classic",
        bg: Color::Reset,
        border: Color::White,
        snake_head: Color::Green,
        snake_body: Color::LightGreen,
        food: Color::Red,
        obstacle: Color::DarkGray,
        ghost: Color::Cyan,
        multiplier: Color::Magenta,
        combo: Color::Yellow,
    },
    // Dark Mode
    Theme {
        name: "Dark",
        bg: Color::Black,
        border: Color::Gray,
        snake_head: Color::Blue,
        snake_body: Color::LightBlue,
        food: Color::LightRed,
        obstacle: Color::White,
        ghost: Color::LightCyan,
        multiplier: Color::LightMagenta,
        combo: Color::Rgb(255, 215, 0), // Gold color
    },
    // Retro (Matrix-ish)
    Theme {
        name: "Retro",
        bg: Color::Black,
        border: Color::Green,
        snake_head: Color::LightGreen,
        snake_body: Color::Green,
        food: Color::Green, // Everything is green!
        obstacle: Color::DarkGray,
        ghost: Color::White,
        multiplier: Color::LightYellow,
        combo: Color::White,
    },
    // Neon
    Theme {
        name: "Neon",
        bg: Color::Black,
        border: Color::Cyan,
        snake_head: Color::LightMagenta,
        snake_body: Color::Magenta,
        food: Color::LightYellow,
        obstacle: Color::Blue,
        ghost: Color::White,
        multiplier: Color::Red,
        combo: Color::LightCyan,
    },
];

// Modern color palette (for elements not in Theme struct)
pub const COLOR_SCORE: Color = Color::Rgb(242, 199, 89);
pub const COLOR_TEXT_PRIMARY: Color = Color::Rgb(242, 245, 247);
pub const COLOR_TEXT_SECONDARY: Color = Color::Rgb(179, 186, 194);
