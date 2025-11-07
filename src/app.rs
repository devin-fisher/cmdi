use crate::event::{Event, EventHandler};
use crate::screens::{RenderContext, Screen, ScreenStack, Transition};
use crate::theme::{DefaultTheme, UiTheme};
use color_eyre::eyre::eyre;
use log::debug;
use ratatui::DefaultTerminal;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;

/// Application.
pub struct App {
    /// Event handler.
    events: EventHandler,
    screens: ScreenStack,
    theme: Box<dyn UiTheme>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(initial_screen: Box<dyn Screen>) -> Self {
        Self {
            events: EventHandler::new(),
            screens: ScreenStack::new(initial_screen),
            // screens_stack_modified: true,
            theme: Box::new(DefaultTheme::default()),
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<String> {
        loop {
            self.handle_render(&mut terminal)?;
            self.screens
                .rendered();

            let result = self.handle_events()?;

            match self.handle_transition(result) {
                Ok(Some(result)) => return Ok(result),
                Ok(None) => {}
                Err(e) => return Err(e),
            }
        }
    }

    fn handle_render(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        if let Some((foreground_screen, background_screens)) = self
            .screens
            .background_foreground_split()
        {
            let ctx = RenderContext::foreground(
                &self.events,
                self.theme
                    .as_ref(),
            );

            let _completed_frame = terminal.try_draw(
                |frame| {
                    // Render background screens
                    for screen in background_screens {
                        if self
                            .screens
                            .modified()
                        {
                            debug!(
                                "render: {}",
                                screen.name()
                            );
                        }
                        screen
                            .render_frame(
                                frame,
                                &ctx.to_background(),
                            )
                            .map(|_| ())
                            .map_err(
                                |err| {
                                    io::Error::new(
                                        io::ErrorKind::Other,
                                        err.to_string(),
                                    )
                                },
                            )?;
                    }

                    if self
                        .screens
                        .modified()
                    {
                        debug!(
                            "render: {}",
                            foreground_screen.name()
                        );
                    }

                    // Render foreground screen
                    foreground_screen
                        .render_frame(
                            frame, &ctx,
                        )
                        .map(|_| ())
                        .map_err(
                            |err| {
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    err.to_string(),
                                )
                            },
                        )
                },
            )?;

            if let Some(info) = foreground_screen.delegation(&ctx) {
                if self
                    .screens
                    .modified()
                {
                    debug!(
                        "delegate: {}",
                        foreground_screen.name()
                    );
                }

                let _ = &self
                    .events
                    .disable_events_thread();
                terminal.set_cursor_position(info.pos)?;
                disable_raw_mode()?;

                foreground_screen.delegate_terminal(&ctx)?;

                enable_raw_mode()?;
                terminal.clear()?;
                let _ = &self
                    .events
                    .enable_event_thread();
                Ok(())
            } else {
                Ok(())
            }
        } else {
            Err(eyre!("No screens found"))
        }
    }

    fn handle_events(&mut self) -> color_eyre::Result<Transition> {
        match self
            .events
            .next()?
        {
            event @ Event::Tick => match self
                .screens
                .active_screen()
            {
                Some(screen) => screen.update(event),
                None => Err(eyre!("Screen MUST be found")),
            },
            Event::Exit => Ok(Transition::Exit(String::from("Keyboard requested exit"))),
            event => {
                debug!(
                    "event: {:?}",
                    event
                );
                match self
                    .screens
                    .active_screen()
                {
                    Some(screen) => {
                        debug!(
                            "handle event: {}",
                            screen.name()
                        );
                        screen.update(event)
                    }
                    None => Err(eyre!("Screen MUST be found")),
                }
            }
        }
    }

    fn handle_transition(&mut self, transition: Transition) -> color_eyre::Result<Option<String>> {
        match transition {
            Transition::Exit(msg) => Err(
                eyre!(
                    format!(
                        "exit - {}",
                        msg
                    )
                ),
            ),
            Transition::Continue => Ok(None),
            Transition::Push(screen) => {
                debug!(
                    "push to: {}",
                    screen.name()
                );
                self.screens
                    .push_screen(screen);
                // self.screens_stack_modified = true;
                Ok(None)
            }
            Transition::Complete(return_value) => {
                self.screens
                    .pop_screen()
                    .inspect(
                        |finished_screen| {
                            debug!(
                                "pop off: {}",
                                finished_screen.name()
                            )
                        },
                    );
                if self
                    .screens
                    .is_empty()
                {
                    // App is complete
                    match return_value {
                        _ => Ok(None),
                    }
                } else {
                    match self
                        .screens
                        .active_screen()
                    {
                        Some(screen) => {
                            debug!(
                                "process return: {}",
                                screen.name()
                            );
                            screen.process(return_value)?;
                            Ok(None)
                        }
                        None => Err(eyre!("Screen MUST be found")),
                    }
                }
            }
        }
    }
}
