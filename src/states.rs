use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Progressing,
    Completed,
}

pub struct ColorScheme;

impl ColorScheme {
    pub const IDLE: Color = Color::Rgb(255, 255, 255);
    pub const PROGRESSING: Color = Color::Rgb(255, 255, 0);
    pub const COMPLETED: Color = Color::Rgb(0, 255, 0);

    pub fn state_to_color(state: &State) -> Color {
        match state {
            State::Idle => Self::IDLE,
            State::Progressing => Self::PROGRESSING,
            State::Completed => Self::COMPLETED
        }
        
    }
}