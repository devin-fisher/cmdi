use crate::event::Event;
use crate::opencli::usage::StringWriter;
use crate::opencli::v0_1::OptionElement;
use crate::screens::input_screen::fzf_select::Item::{CmdOption, Plain};
use crate::screens::Rendering::Complete;
use crate::screens::Return::Noop;
use crate::screens::{DeligationInfo, RenderContext, Rendering, Return, Screen, Transition};
use fzf_wrapped::{run_with_output, Fzf, Layout};
use ratatui::layout::Position;
use ratatui::Frame;

pub struct FzfSelectScreen {
    options: Vec<Item>,
    ran: bool,
}

#[allow(dead_code)]
enum Item {
    Plain(String),
    CmdOption(OptionElement)
}

impl FzfSelectScreen {
    #[allow(dead_code)]
    pub fn new_with_strings<T: AsRef<str>>(options: &Vec<T>) -> Box<dyn Screen> {
        let screen = Self {
            options: options.iter().map(|x| Plain(x.as_ref().to_owned())).collect(),
            ran: false,
        };

        Box::new(screen)
    }
    pub fn new_with_options(options: &Vec<OptionElement>) -> Box<dyn Screen> {
        let screen = Self {
            options: options.iter().map(|x| CmdOption(x.to_owned())).collect(),
            ran: false,
        };

        Box::new(screen)
    }
}

impl Screen for FzfSelectScreen {
    fn update(&mut self, event: Event) -> color_eyre::Result<Transition> {
        self.ran = true;

        let rtn = match event {
            Event::Selection(selections) => {
                match selections.is_empty() {
                    true => Transition::Complete(Noop),
                    false => Transition::Complete(
                        Return::Selection(selections)
                    )
                }
            }
            _ => Transition::Continue,
        };

        Ok(rtn)
    }

    fn process(&mut self, _result: Return) -> color_eyre::Result<()> {
        Ok(())
    }

    fn render_frame(
        &self,
        _frame: &mut Frame,
        _ctx: &RenderContext
    ) -> color_eyre::Result<Rendering> {
        Ok(Complete)
    }

    fn delegation(&self, _ctx: &RenderContext) -> Option<DeligationInfo> {
        if !self.ran {
            Some(
                DeligationInfo {
                    pos: Position {
                        x: 0,
                        y: 1,
                    },
                },
            )
        } else {
            None
        }
    }

    fn delegate_terminal(
        &self,
        ctx: &RenderContext
    ) -> color_eyre::Result<()> {
        let fzf = Fzf::builder()
            .custom_args(
                vec![
                    "--height=99%",
                    "--delimiter=\t",
                    "--color=border:#808080",
                    "--color=gutter:-1",
                    "--border=rounded",
                    "--pointer=>>",
                    "--marker=>",
                    "--prompt=>>",
                    "--padding=1",
                    "--margin=1",
                    "--accept-nth=1",
                    "--with-nth=2",
                    "--ansi",
                ],
            )
            .layout(Layout::Reverse)
            .build()?;

        let options: Vec<String> = self
            .options
            .iter()
            .enumerate()
            .map(
                |(i, o)| {
                    format!(
                        "{}\t{}",
                        i,
                        match o {
                            Plain(str) => str.to_owned(),
                            CmdOption(option) => {
                                StringWriter::spec_to_usage_line(
                                    option,
                                    ctx.theme.usage_styles()
                                )
                            }
                        }
                        .replace(
                            "\n", " -- "
                        )
                    )
                },
            )
            .collect();

        let selections: Vec<usize> = run_with_output(
            fzf, options,
        )
        .unwrap_or_default()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

        ctx.event.send(Event::Selection(selections));

        Ok(())
    }
}
