use crate::task::Priority;
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::style::{Color, Modifier, Style};

pub const PINK_COLOR: Color = Color::Rgb(255, 192, 203);

pub const OVERDUE_TASK_FG: Color = Color::LightRed;
pub const TODAY_TASK_FG: Color = Color::White;
pub const FUTURE_TASK_FG: Color = Color::LightGreen;
pub const NO_DATE_TASK_FG: Color = TODAY_TASK_FG;
pub const DESCRIPTION_KEY_COLOR: Color = Color::Blue;
pub const DESCRIPTION_VALUE_COLOR: Color = Color::White;
pub const NORMAL_ROW_BG: Color = SLATE.c950;
pub const PROVIDER_COLORS: &[Color] = &[
    Color::Green,
    Color::Magenta,
    Color::Cyan,
    Color::Yellow,
    Color::Blue,
    Color::Red,
];

pub const ACTIVE_BLOCK_STYLE: Style = Style::new().fg(SLATE.c100).bg(GREEN.c800);
pub const INACTIVE_BLOCK_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn priority_color(p: &Priority) -> Color {
    match p {
        Priority::Lowest => Color::DarkGray,
        Priority::Low => Color::Gray,
        Priority::Normal => Color::LightGreen,
        Priority::Medium => PINK_COLOR,
        Priority::High => Color::LightRed,
        Priority::Highest => Color::Red,
    }
}
