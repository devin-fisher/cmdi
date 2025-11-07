use crate::builder::{Action, Token};
use crate::event::Event;
use crate::screens::Transition::{Continue, Push};
use crate::screens::action_screen::add_arguments::AddArgumentsScreen;
use crate::screens::action_screen::create_option::SelectOptionScreen;
use crate::screens::builder_screen::model::BuilderScreen;
use crate::screens::{Return, Transition};
use log::warn;
use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::KeyCode;

impl BuilderScreen {
    fn delegate_action(&self, action: &Action, token: &Token) -> color_eyre::Result<Transition> {
        let result = match token {
            Token::CommandToken {
                ctx,
            } => match action {
                Action::InsertOptionBelow => match &ctx.options {
                    None => Continue,
                    Some(options) => Push(
                        SelectOptionScreen::new(
                            &options, None, &action,
                        ),
                    ),
                },
                _ => Continue,
            },
            Token::SubCommandToken {
                ctx,
                ..
            } => match action {
                Action::InsertOptionBelow => {
                    match &ctx
                        .spec
                        .options
                    {
                        None => Continue,
                        Some(options) => Push(
                            SelectOptionScreen::new(
                                &options,
                                Some(ctx),
                                &action,
                            ),
                        ),
                    }
                }
                _ => Continue,
            },
            Token::OptionToken {
                ..
            } => match action {
                Action::InsertArgument => Push(
                    AddArgumentsScreen::new(
                        token, action,
                    ),
                ),
                _ => Continue,
            },
            _ => {
                warn!(
                    "Unhandled action: {:?} for {}",
                    action, token
                );
                Continue
            }
        };

        Ok(result)
    }
    pub fn update(&mut self, event: Event) -> color_eyre::Result<Transition> {
        match event {
            Event::Crossterm(event) => match event {
                Key(key_event) => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => Ok(Transition::Complete(Return::Noop)),
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                        self.builder
                            .selected_up();
                        Ok(Continue)
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                        self.builder
                            .selected_down();
                        Ok(Continue)
                    }
                    KeyCode::Char('h') => {
                        // Display help screen
                        Ok(Continue)
                    }
                    KeyCode::Char(key_char) => match self
                        .builder
                        .token_at_selected()
                    {
                        None => Ok(Continue), // Should never happen, means there is no selected token
                        Some(token) => {
                            if let Some(action) = Action::keybinding_to_action(
                                key_char,
                                self.builder
                                    .available_actions(),
                            ) {
                                self.delegate_action(
                                    &action, &token,
                                )
                            } else {
                                Ok(Continue) // No keybinding for current token (normal)
                            }
                        }
                    },
                    _key_code => Ok(Continue),
                },
                _ => Ok(Continue),
            },
            _ => Ok(Continue),
        }
    }

    pub fn process(&mut self, return_value: Return) -> color_eyre::Result<()> {
        match return_value {
            Return::TokenAction(token, action) => self
                .builder
                .do_token_action(
                    token, action,
                ),
            _ => Ok(()),
        }
    }
}
