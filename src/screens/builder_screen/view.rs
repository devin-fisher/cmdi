use crate::builder::Token::{
    ArgumentToken, CommandToken, OptionToken, PlaceholderToken, SubCommandToken,
};
use crate::builder::{Builder, Token};
use crate::opencli::usage::WidgetWriter;
use crate::screens::builder_screen::model::BuilderScreen;
use crate::screens::{KeyBinding, KeyBindingType, RenderContext, RenderLayer, Rendering};
use itertools::Itertools;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Wrap};
use std::iter;
use std::iter::once;

impl BuilderScreen {
    pub(crate) fn render_to_frame(
        &self,
        frame: &mut Frame,
        ctx: &RenderContext,
    ) -> color_eyre::Result<Rendering> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                vec![
                    Constraint::Percentage(62),
                    Constraint::Percentage(38),
                ],
            )
            .split(frame.area());

        let builder = layout[0];
        let info = layout[1];

        self.render_builder_block(
            builder, frame, ctx,
        )?;
        self.render_builder_info(
            info,
            frame,
            frame.count(),
            ctx,
        )?;

        Ok(Rendering::Complete)
    }
}

impl BuilderScreen {
    fn render_builder_block(
        &self,
        space: Rect,
        frame: &mut Frame,
        ctx: &RenderContext,
    ) -> color_eyre::Result<Rendering> {
        let builder_block = Block::bordered()
            .border_style(
                ctx.theme
                    .screen_styles()
                    .boarder,
            )
            .border_type(BorderType::Rounded)
            .title_top(
                Line::from(
                    format!(
                        " {} ",
                        "cmdi",
                    ),
                )
                .style(
                    ctx.theme
                        .screen_styles()
                        .default,
                )
                .right_aligned(),
            )
            .title_bottom(
                match ctx.layer {
                    RenderLayer::Foreground => self
                        .render_keys(ctx)
                        .centered(),
                    RenderLayer::Background => Line::default(),
                },
            );

        let builder_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Fill(100), // for the list, takes the remaining space
                ],
            )
            .split(builder_block.inner(space));

        frame.render_widget(
            builder_block,
            space,
        );

        let token_list = builder_layout[0];

        self.render_builder_tokens(
            token_list, frame, ctx,
        )?;

        Ok(Rendering::Complete)
    }

    fn render_builder_tokens(
        &self,
        space: Rect,
        frame: &mut Frame,
        ctx: &RenderContext,
    ) -> color_eyre::Result<Rendering> {
        let list = List::from(&self.builder)
            .highlight_symbol(">> ")
            .highlight_style(
                ctx.theme
                    .screen_styles()
                    .highlight,
            );

        frame.render_stateful_widget(
            list,
            space,
            &mut ListState::from(&self.builder),
        );

        Ok(Rendering::Complete)
    }

    fn render_builder_info(
        &self,
        space: Rect,
        frame: &mut Frame,
        _frame_count: usize,
        ctx: &RenderContext,
    ) -> color_eyre::Result<Rendering> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                vec![
                    Constraint::Percentage(62),
                    Constraint::Percentage(38),
                ],
            )
            .split(space);

        let doc_space = layout[0];
        let details_space = layout[1];

        let doc_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(
                ctx.theme
                    .screen_styles()
                    .boarder,
            )
            .title_top(
                Line::from(
                    format!(
                        "[{}]",
                        t!("headings.doc")
                    ),
                )
                .style(
                    ctx.theme
                        .screen_styles()
                        .default,
                )
                .centered(),
            );

        let usage = match self
            .builder
            .token_at_selected()
        {
            None => Text::from(""),
            Some(token) => WidgetWriter::token_to_usage_text(
                token,
                ctx.theme
                    .usage_styles(),
            ),
        };

        let doc_paragraph = Paragraph::new(usage)
            .block(doc_block)
            .wrap(
                Wrap {
                    trim: false,
                },
            )
            .scroll(
                (
                    0, 0,
                ),
            );

        let details_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(
                ctx.theme
                    .screen_styles()
                    .boarder,
            )
            .title_top(
                Line::from(
                    format!(
                        "[{}]",
                        t!("headings.details")
                    ),
                )
                .style(
                    ctx.theme
                        .screen_styles()
                        .default,
                )
                .centered(),
            )
            .title_bottom(
                Line::from(
                    format!(
                        "[{:05}]",
                        _frame_count
                    ),
                )
                .right_aligned(),
            );

        frame.render_widget(
            doc_paragraph,
            doc_space,
        );
        frame.render_widget(
            details_block,
            details_space,
        );

        Ok(Rendering::Complete)
    }

    fn render_keys(&'_ self, ctx: &RenderContext) -> Line<'_> {
        // confident that intersperse will be same when stabilize
        #[allow(unstable_name_collisions)]
        let keybindings: Vec<_> = once(
            BuilderScreen::styled_keybind_hint(
                (
                    'q',
                    KeyBindingType::Exit,
                ),
                t!("action_hints.quit")
                    .to_string()
                    .as_str(),
                ctx,
            ),
        )
        .chain(
            self.builder
                .available_actions()
                .iter()
                .sorted()
                .map(
                    |action| {
                        BuilderScreen::styled_keybind_hint(
                            action.keybinding(),
                            action
                                .display_name()
                                .as_str(),
                            ctx,
                        )
                    },
                )
                .collect::<Vec<_>>(),
        )
        .chain(
            once(
                BuilderScreen::styled_keybind_hint(
                    (
                        'h',
                        KeyBindingType::Informative,
                    ),
                    t!("action_hints.help")
                        .to_string()
                        .as_str(),
                    ctx,
                ),
            ),
        )
        .intersperse(
            vec![
                Span::styled(
                    " ",
                    ctx.theme
                        .screen_styles()
                        .default,
                ),
            ],
        )
        .flatten()
        .collect();

        Line::from(keybindings)
    }

    fn styled_keybind_hint(
        keybinding: (
            char,
            KeyBindingType,
        ),
        display_name: &str,
        ctx: &RenderContext,
    ) -> Vec<Span<'static>> {
        vec![
            Span::styled(
                "[",
                ctx.theme
                    .screen_styles()
                    .default,
            ),
            Span::styled(
                keybinding
                    .0
                    .to_string(),
                ctx.theme
                    .screen_styles()
                    .action_to_style(&keybinding.1),
            ),
            Span::styled(
                "]",
                ctx.theme
                    .screen_styles()
                    .default,
            ),
            Span::styled(
                " ",
                ctx.theme
                    .screen_styles()
                    .default,
            ),
            Span::styled(
                display_name.to_owned(),
                ctx.theme
                    .screen_styles()
                    .default,
            ),
        ]
    }
}

impl<'a> From<&Token> for Text<'a> {
    fn from(value: &Token) -> Self {
        let spans = match value {
            OptionToken {
                ctx: _ctx,
                spec,
                arg,
                details: _details,
            } => vec![
                Span::from(
                    format!(
                        "{} {}",
                        spec.name
                            .clone(),
                        arg.join(",")
                    ),
                ),
            ], // FIXME
            SubCommandToken {
                ctx: context,
                ..
            } => vec![
                Span::from(
                    context
                        .spec
                        .name
                        .to_owned(),
                ),
            ],
            ArgumentToken {
                ctx: _context,
                arg,
                ..
            } => vec![Span::from(arg.clone())],
            PlaceholderToken {
                ..
            } => vec![Span::from("")],
            CommandToken {
                ctx,
            } => vec![
                Span::from(
                    ctx.info
                        .title
                        .to_owned(),
                ),
            ],
        };

        Line::from_iter(iter::once(Span::from("  ".repeat(value.level()))).chain(spans)).into()
    }
}

impl<'a> From<&Builder> for List<'a> {
    fn from(builder: &Builder) -> Self {
        List::new(
            builder
                .tokens()
                .iter()
                .map(|value| ListItem::new(value))
                .collect::<Vec<ListItem>>(),
        )
    }
}

impl From<&Builder> for ListState {
    fn from(value: &Builder) -> Self {
        ListState::default().with_selected(Some(value.pos_at_selected()))
    }
}
