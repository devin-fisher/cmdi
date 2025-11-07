mod action_screen;
pub(crate) mod builder_screen;
mod input_screen;

use crate::builder::{Action, Token};
use crate::event::{Event, EventHandler};
use crate::theme::UiTheme;
use ratatui::Frame;
use ratatui::layout::Position;
use std::any::{Any, type_name};

pub enum Transition {
    Continue,              // keep current subscreen active
    Push(Box<dyn Screen>), // push a new child subscreen
    Complete(Return),      // subscreen exits with given result
    Exit(String),          // exit application with given reason
}

pub enum Return {
    TokenAction(
        Token,
        Action,
    ),
    Noop,
    Selection(Vec<usize>),
    InputString(String),
}

#[allow(dead_code)]
pub struct RenderContext<'a> {
    event: &'a EventHandler,
    theme: &'a dyn UiTheme,
    layer: RenderLayer,
}

pub enum RenderLayer {
    Foreground,
    Background,
}

impl<'a> RenderContext<'a> {
    pub fn foreground(event: &'a EventHandler, theme: &'a dyn UiTheme) -> Self {
        Self {
            event,
            theme,
            layer: RenderLayer::Foreground,
        }
    }

    pub fn background(event: &'a EventHandler, theme: &'a dyn UiTheme) -> Self {
        Self {
            event,
            theme,
            layer: RenderLayer::Background,
        }
    }

    pub fn to_background(&self) -> Self {
        Self::background(self.event, self.theme)
    }
}

#[allow(dead_code)]
pub enum Rendering {
    Complete,
    Error(String),
}

pub struct DeligationInfo {
    pub(crate) pos: Position,
}

pub trait Screen: Any {
    fn name(&self) -> &'static str {
        type_name::<Self>()
            .split("::")
            .last()
            .unwrap()
    }
    fn update(&mut self, event: Event) -> color_eyre::Result<Transition>;
    fn process(&mut self, return_value: Return) -> color_eyre::Result<()>;
    fn render_frame(&self, frame: &mut Frame, ctx: &RenderContext) -> color_eyre::Result<Rendering>;

    fn delegation(&self, ctx: &RenderContext) -> Option<DeligationInfo>;
    fn delegate_terminal(
        &self,
        ctx: &RenderContext
    ) -> color_eyre::Result<()>;
}

pub trait KeyBinding {
    fn rank(&self) -> usize;
    fn keybinding(
        &self,
    ) -> (
        char,
        KeyBindingType,
    );
    fn display_name(&self) -> String;
}

pub enum KeyBindingType {
    Exit,
    Modifier,
    Informative,
}


pub struct ScreenStack {
    screens: Vec<Box<dyn Screen>>,
    screens_stack_modified: bool,
}

impl ScreenStack {
    pub(crate) fn background_foreground_split(&self) -> Option<(&Box<dyn Screen>, &[Box<dyn Screen>])> {//&[Box<dyn Screen>] {
        self.screens.as_slice().split_last()
    }

    pub(crate) fn active_screen(&mut self) -> Option<&mut Box<dyn Screen>> {
        self.screens.last_mut()
    }

    pub(crate) fn push_screen(&mut self, screen: Box<dyn Screen>) {
        self.screens.push(screen);
        self.screens_stack_modified = true;
    }

    pub(crate) fn pop_screen(&mut self) -> Option<Box<dyn Screen>> {
        self.screens_stack_modified = true;
        self.screens.pop()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.screens.is_empty()
    }

    pub(crate) fn modified(&self) -> bool {
        self.screens_stack_modified
    }

    pub(crate) fn rendered(&mut self) {
        self.screens_stack_modified = false;
    }
}

impl ScreenStack {
    pub(crate) fn new(initial_screens: Box<dyn Screen>) -> Self {
        Self {
            screens: vec![initial_screens],
            screens_stack_modified: true,
        }
    }
}