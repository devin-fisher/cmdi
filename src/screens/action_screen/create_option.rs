use crate::builder::{Action, CommandContext, Token};
use crate::event::{Event};
use crate::opencli::v0_1::OptionElement;
use crate::screens::input_screen::fzf_select::FzfSelectScreen;
use crate::screens::{DeligationInfo, RenderContext, Rendering, Return, Screen, Transition};
use log::{warn};
use ratatui::Frame;

pub struct SelectOptionScreen {
    options: Vec<OptionElement>,
    ctx: Option<CommandContext>,
    action: Action,
    complete: bool,
    selection: Option<OptionElement>,
}

impl SelectOptionScreen {
    fn build_token(&self) -> Token {
        Token::OptionToken {
            ctx: self
                .ctx
                .clone(),
            spec: self.selection.clone().unwrap(),
            arg: vec![],
            details: vec![],
        }
    }
}

impl SelectOptionScreen {
    pub fn new(options: &Vec<OptionElement>, ctx: Option<&CommandContext>, action: &Action) -> Box<dyn Screen> {
        let screen = Self {
            options: options.clone(),
            ctx: ctx.cloned(),
            action: action.clone(),
            complete: false,
            selection: None,
        };

        Box::new(screen)
    }
}
impl Screen for SelectOptionScreen {
    fn update(&mut self, event: Event) -> color_eyre::Result<Transition> {
        match event {
            Event::Exit => Ok(Transition::Exit("Exit Event".to_string())),
            _ => {
                if !self.complete {
                    Ok(Transition::Push(FzfSelectScreen::new_with_options(&self.options)))
                } else {
                    if self.selection.is_none() {
                        Ok(Transition::Complete(Return::Noop))
                    }
                    else {
                        Ok(
                            Transition::Complete(
                                Return::TokenAction(
                                    self.build_token(),
                                    self.action
                                        .clone(),
                                ),
                            ),
                        )
                    }
                }
            }
        }
    }

    fn process(&mut self, return_value: Return) -> color_eyre::Result<()> {
        match return_value {
            Return::Noop => {
                self.complete = true;
                Ok(())
            },
            Return::Selection(selections) => {
                self.complete = true;

                if selections.len() >= 2 {
                    warn!("Selection was greater than 1")
                }
                self.selection = selections
                    .first()
                    .and_then(
                        |s| {
                            self.options
                                .get(*s)
                        },
                    )
                    .cloned();

                Ok(())
            }
            _ => Ok(())
        }
    }

    fn render_frame(
        &self,
        _frame: &mut Frame,
        _ctx: &RenderContext
    ) -> color_eyre::Result<Rendering> {
        Ok(Rendering::Complete)
    }

    fn delegation(&self, _ctx: &RenderContext) -> Option<DeligationInfo> {
        None
    }

    fn delegate_terminal(
        &self,
        _ctx: &RenderContext
    ) -> color_eyre::Result<()> {
        Ok(())
    }
}
