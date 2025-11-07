use crate::builder::Token::{ArgumentToken, CommandToken, SubCommandToken};
use crate::builder::{Builder, CommandContext};
use crate::event::Event;
use crate::opencli::v0_1::V0_1;
use crate::screens::{DeligationInfo, RenderContext, Rendering, Return, Screen, Transition};
use ratatui::Frame;

pub struct BuilderScreen {
    pub(crate) builder: Builder,
}

impl Screen for BuilderScreen {
    fn update(&mut self, event: Event) -> color_eyre::Result<Transition> {
        self.update(event)
    }

    fn process(&mut self, return_value: Return) -> color_eyre::Result<()> {
        self.process(return_value)
    }

    fn render_frame(
        &self,
        frame: &mut Frame,
        ctx: &RenderContext
    ) -> color_eyre::Result<Rendering> {
        self.render_to_frame(
            frame, &ctx
        )
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
impl BuilderScreen {
    pub fn demo(command_spec: V0_1) -> Box<dyn Screen> {
        let get_command_element = command_spec
            .commands
            .as_ref()
            .unwrap()
            .first()
            .map(|c| c.to_owned())
            .unwrap();

        let screen = Self {
            builder: Builder::new_demo(
                command_spec.clone(),
                vec![
                    CommandToken {
                        ctx: command_spec,
                    },
                    SubCommandToken {
                        ctx: CommandContext::new(
                            1,
                            get_command_element.clone(),
                        ),
                        details: vec![],
                    },
                    ArgumentToken {
                        ctx: Some(
                            CommandContext::new(
                                1,
                                get_command_element.clone(),
                            ),
                        ),
                        spec: get_command_element
                            .arguments
                            .as_ref()
                            .unwrap()
                            .first()
                            .map(|c| c.to_owned())
                            .unwrap(),
                        arg: "event".to_string(),
                        details: vec![],
                    },
                ],
            ),
        };

        Box::new(screen)
    }
}
