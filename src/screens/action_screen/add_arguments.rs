use crate::builder::{Action, Token};
use crate::event::Event;
use crate::screens::input_screen::input::InputScreen;
use crate::screens::input_screen::noop::NoopScreen;
use crate::screens::{DeligationInfo, RenderContext, Rendering, Return, Screen, Transition};
use ratatui::Frame;

pub struct AddArgumentsScreen {
    token: Token,
    action: Action,
    complete: bool,
}
impl AddArgumentsScreen {
    pub fn new(token: &Token, action: &Action) -> Box<dyn Screen> {
        match token {
            Token::OptionToken {
                ..
            } => Box::new(
                Self {
                    token: token.clone(),
                    action: action.clone(),
                    complete: false,
                },
            ),
            // Token::ArgumentToken { .. } => {}
            _ => NoopScreen::new(),
        }
    }
}
impl Screen for AddArgumentsScreen {
    fn update(&mut self, event: Event) -> color_eyre::Result<Transition> {
        match event {
            Event::Exit => Ok(Transition::Exit("Exit Event".to_string())),
            _ => {
                if !self.complete {
                    self.complete = true;
                    Ok(
                        Transition::Push(
                            InputScreen::new(
                                &self
                                    .token
                                    .args()
                                    .join(""),
                            ),
                        ),
                    )
                } else {
                    Ok(
                        Transition::Complete(
                            Return::TokenAction(
                                self.token
                                    .clone(),
                                self.action
                                    .clone(),
                            ),
                        ),
                    )
                }
            }
        }
    }

    fn process(&mut self, return_value: Return) -> color_eyre::Result<()> {
        match return_value {
            Return::Noop => Ok(()),
            Return::InputString(input) => {
                match &mut self.token {
                    Token::PlaceholderToken => {}
                    Token::CommandToken {
                        ..
                    } => {}
                    Token::OptionToken {
                        arg,
                        ..
                    } => {
                        arg.clear();
                        arg.push(input);
                    }
                    Token::SubCommandToken {
                        ..
                    } => {}
                    Token::ArgumentToken {
                        ..
                    } => {}
                }
                Ok(())
            }
            _ => Ok(()),
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
