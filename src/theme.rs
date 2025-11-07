use crate::screens::KeyBindingType;
use ratatui::crossterm::style::{Color, ContentStyle as Style, Stylize};

#[derive(Clone)]
pub struct ScreenStyle {
    pub default: Style,
    pub highlight: Style,

    pub boarder: Style,

    pub key_exit: Style,
    pub key_informative: Style,
    pub key_modifier: Style,
}

impl ScreenStyle {
    pub fn action_to_style(&self, key_binding_type: &KeyBindingType) -> Style {
        match key_binding_type {
            KeyBindingType::Modifier => self.key_modifier,
            KeyBindingType::Exit => self.key_exit,
            KeyBindingType::Informative => self.key_informative,
        }
    }
}

#[derive(Clone)]
pub struct UsageStyle {
    pub default: Style,

    pub header: Style,
    pub name: Style,
    pub description: Style,
    pub details: Style,
}

pub struct DefaultTheme {
    screen_style: ScreenStyle,
    usage_style: UsageStyle,
}
impl DefaultTheme {
    pub fn default() -> Self {
        Self {
            screen_style: ScreenStyle {
                default: Style::new().with(Color::White),

                highlight: Style::new().on(Color::Blue),

                boarder: Style::new().with(Color::DarkGrey),

                key_exit: Style::new().with(Color::Red),
                key_informative: Style::new().with(Color::Yellow),
                key_modifier: Style::new().with(Color::Cyan),
            },

            usage_style: UsageStyle {
                default: Default::default(),
                header: Style::new().with(Color::Yellow),
                name: Style::new().with(Color::Cyan),
                description: Default::default(),
                details: Style::new().with(Color::Magenta),
            },
        }
    }
}

impl UiTheme for DefaultTheme {
    fn screen_styles(&self) -> &ScreenStyle {
        &self.screen_style
    }

    fn usage_styles(&self) -> &UsageStyle {
        &self.usage_style
    }
}

pub trait UiTheme {
    fn screen_styles(&self) -> &ScreenStyle;
    fn usage_styles(&self) -> &UsageStyle;
}
