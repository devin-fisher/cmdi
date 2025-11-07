use crate::event::Event;
use crate::screens::{DeligationInfo, RenderContext, Rendering, Return, Screen, Transition};
use ratatui::Frame;
use ratatui::crossterm::event::Event::Key;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Clear, Paragraph};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use crate::screens::Return::{Noop, InputString};

pub struct InputScreen {
    input: Input,
}

impl InputScreen {
    pub(crate) fn new(initial_val: &String) -> Box<dyn Screen> {
        let screen = Self {
            input: Input::new(initial_val.to_owned()),
        };

        Box::new(screen)
    }
}

impl Screen for InputScreen {
    fn update(&mut self, event: Event) -> color_eyre::Result<Transition> {
        let transition = match event {
            Event::Crossterm(event) => match event {
                Key(KeyEvent {
                    code,
                    ..
                }) => match code {
                    KeyCode::Esc => Transition::Complete(Noop),
                    KeyCode::Enter => Transition::Complete(
                        InputString(
                            self.input
                                .to_string(),
                        ),
                    ),
                    _ => {
                        self.input
                            .handle_event(&event);
                        Transition::Continue
                    }
                },
                _ => Transition::Continue,
            },
            _ => Transition::Continue,
        };
        Ok(transition)
    }

    fn process(&mut self, _result: Return) -> color_eyre::Result<()> {
        // Can't process returns
        Ok(())
    }

    fn render_frame(
        &self,
        frame: &mut Frame,
        _ctx: &RenderContext
    ) -> color_eyre::Result<Rendering> {
        let [
            _header_area,
            input_area,
        ] = Layout::vertical(
            [
                Constraint::Length(1),
                Constraint::Length(3),
            ],
        )
        .areas(frame.area());

        frame.render_widget(
            Clear, input_area,
        );

        self.render_input(
            frame, input_area,
        );

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

impl InputScreen {
    fn render_input(&self, frame: &mut Frame, area: Rect) {
        // keep 2 for borders and 1 for cursor
        let width = area
            .width
            .max(3)
            - 3;
        let scroll = self
            .input
            .visual_scroll(width as usize);
        let style = Style::default();
        let input = Paragraph::new(
            self.input
                .value(),
        )
        .style(style)
        .scroll(
            (
                0,
                scroll as u16,
            ),
        )
        .block(Block::bordered().title("Input"));
        frame.render_widget(
            input, area,
        );

        let x = self
            .input
            .visual_cursor()
            .max(scroll)
            - scroll
            + 1;
        frame.set_cursor_position(
            (
                area.x + x as u16,
                area.y + 1,
            ),
        )
    }
}
