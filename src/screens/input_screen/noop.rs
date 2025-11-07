use crate::event::Event;
use crate::screens::{DeligationInfo, RenderContext, Rendering, Return, Screen, Transition};
use ratatui::Frame;

pub struct NoopScreen;

impl NoopScreen {
    pub fn new() -> Box<dyn Screen> {
        Box::new(NoopScreen)
    }
}
impl Screen for NoopScreen {
    fn update(&mut self, _event: Event) -> color_eyre::Result<Transition> {
        Ok(Transition::Complete(Return::Noop))
    }

    fn process(&mut self, _return_value: Return) -> color_eyre::Result<()> {
        Ok(())
    }

    fn render_frame(&self, _frame: &mut Frame, _ctx: &RenderContext) -> color_eyre::Result<Rendering> {
        Ok(Rendering::Complete)
    }

    fn delegation(&self, _ctx: &RenderContext) -> Option<DeligationInfo> {
        None
    }

    fn delegate_terminal(&self, _ctx: &RenderContext) -> color_eyre::Result<()> {
        Ok(())
    }
}