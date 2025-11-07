use crate::builder::Token;
use crate::builder::Token::{
    ArgumentToken, CommandToken, OptionToken, PlaceholderToken, SubCommandToken,
};
use crate::opencli::usage::UsageStyleClass::*;
use crate::opencli::v0_1::{
    ArgumentElement, Arity, CommandElement, ExitCodeElement, OptionElement, V0_1,
};
use crate::theme::UsageStyle;
use ratatui::text::{Line, Span, Text};
use std::fmt::Display;
use std::ops::Add;

pub trait Usage {
    fn usage<W: UsageWriter>(&self, out: &mut W, indent: usize);
    fn usage_line<W: UsageWriter>(&self, out: &mut W, indent: usize);
}

#[derive(Debug, Clone, Copy)]
pub enum UsageStyleClass {
    Header,
    Name,
    Description,
    Details,
}

pub trait UsageWriter {
    fn write<T: AsRef<str>>(&mut self, text: T, style: Option<UsageStyleClass>);
    fn write_opt<T: AsRef<str>>(&mut self, text: &Option<T>, style: Option<UsageStyleClass>) {
        match text {
            None => {}
            Some(t) => self.write(
                t, style,
            ),
        }
    }
    fn newline(&mut self, indent: usize);
}

pub struct WidgetWriter<'a> {
    lines: Vec<Line<'a>>,
    current_line: Vec<Span<'a>>,
    style: UsageStyle,
}
impl<'a> UsageWriter for WidgetWriter<'a> {
    fn write<T: AsRef<str>>(&mut self, text: T, style: Option<UsageStyleClass>) {
        let owned_text = text
            .as_ref()
            .to_owned();

        self.current_line
            .push(
                match style {
                    None => Span::from(owned_text).style(
                        self.style
                            .default,
                    ),
                    Some(Header) => Span::from(owned_text).style(
                        self.style
                            .header,
                    ),
                    Some(Name) => Span::from(owned_text).style(
                        self.style
                            .name,
                    ),
                    Some(Description) => Span::from(owned_text).style(
                        self.style
                            .description,
                    ),
                    Some(Details) => Span::from(owned_text).style(
                        self.style
                            .details,
                    ),
                },
            );
    }

    fn newline(&mut self, indent: usize) {
        self.lines
            .push(
                Line::from(
                    self.current_line
                        .clone(),
                ),
            );

        self.current_line
            .clear();

        self.current_line
            .push(Span::from(" ".repeat(indent)));
    }
}

impl<'a> WidgetWriter<'a> {
    pub fn new(style: &UsageStyle) -> Self {
        Self {
            lines: vec![],
            current_line: vec![],
            style: style.clone(),
        }
    }
    pub fn into_text(self) -> Text<'a> {
        Text::from(self.lines).add(Line::from(self.current_line))
    }

    pub fn spec_to_text<T: Usage>(spec: &T, style: &UsageStyle) -> Text<'a> {
        let mut out = Self::new(style);
        spec.usage(
            &mut out, 0,
        );
        out.into_text()
    }

    pub fn token_to_usage_text(token: &Token, style: &UsageStyle) -> Text<'a> {
        match token {
            PlaceholderToken => Text::from(""),
            CommandToken {
                ctx,
                ..
            } => Self::spec_to_text(
                ctx, style,
            ),
            OptionToken {
                spec,
                ..
            } => Self::spec_to_text(
                spec, style,
            ),
            SubCommandToken {
                ctx,
                ..
            } => Self::spec_to_text(
                &ctx.spec, style,
            ),
            ArgumentToken {
                spec,
                ..
            } => Self::spec_to_text(
                spec, style,
            ),
        }
    }
}

pub struct StringWriter {
    lines: String,
    style: UsageStyle,
}

impl StringWriter {
    pub fn new(style: &UsageStyle) -> Self {
        Self {
            lines: String::new(),
            style: style.clone(),
        }
    }
    pub fn spec_to_usage_line<T: Usage>(spec: &T, style: &UsageStyle) -> String {
        let mut out = StringWriter::new(style);
        spec.usage_line(
            &mut out, 0,
        );
        out.lines
    }
}

impl UsageWriter for StringWriter {
    fn write<T: AsRef<str>>(&mut self, text: T, style: Option<UsageStyleClass>) {
        let s = match style {
            None => self
                .style
                .default
                .apply(text.as_ref()), //text.as_ref().set_style(self.style.default).to_string(),
            Some(Name) => self
                .style
                .name
                .apply(text.as_ref()),
            Some(Header) => self
                .style
                .header
                .apply(text.as_ref()),
            Some(Description) => self
                .style
                .description
                .apply(text.as_ref()),
            Some(Details) => self
                .style
                .details
                .apply(text.as_ref()),
        }
        .to_string();

        self.lines
            .push_str(s.as_str());
    }

    fn newline(&mut self, indent: usize) {
        self.lines
            .push_str("\n");
        self.lines
            .push_str(
                " ".repeat(indent)
                    .as_ref(),
            );
    }
}

fn foreach<W: UsageWriter, U: Usage>(
    out: &mut W,
    list: &Option<Vec<U>>,
    heading: String,
    indent: usize,
) {
    let indent_next = indent + 2;
    match list {
        None => {}
        Some(list) => {
            if !list.is_empty() {
                out.newline(indent);
            }

            out.write(
                &heading,
                Some(Header),
            );
            out.write(
                ":",
                Some(Header),
            );

            list.iter()
                .for_each(
                    |x| {
                        out.newline(indent_next);
                        x.usage_line(
                            out,
                            indent_next,
                        );
                    },
                );

            if !list.is_empty() {
                out.newline(indent);
            }
        }
    }
}
impl Usage for V0_1 {
    fn usage<W: UsageWriter>(&self, out: &mut W, indent: usize) {
        out.write(
            &self
                .info
                .title,
            Some(Name),
        );
        out.newline(indent);
        out.write_opt(
            &self
                .info
                .description,
            Some(Description),
        );
        out.newline(indent);

        foreach(
            out,
            &self.commands,
            "SUBCOMMANDS".to_string(),
            indent,
        );
        foreach(
            out,
            &self.options,
            "OPTIONS".to_string(),
            indent,
        );
        foreach(
            out,
            &self.arguments,
            "ARGUMENTS".to_string(),
            indent,
        );
        foreach(
            out,
            &self.exit_codes,
            "Exit Codes".to_string(),
            indent,
        );
        foreach(
            out,
            &self.examples,
            "Examples".to_string(),
            indent,
        );
    }

    fn usage_line<W: UsageWriter>(&self, out: &mut W, _indent: usize) {
        out.write(
            &self
                .info
                .title,
            Some(Name),
        );
        out.write(
            " -- ", None,
        );
        out.write_opt(
            &self
                .info
                .description,
            Some(Description),
        );
    }
}
impl Usage for CommandElement {
    fn usage<W: UsageWriter>(&self, out: &mut W, indent: usize) {
        out.write(
            &self.name,
            Some(Name),
        );
        out.newline(indent);
        out.write_opt(
            &self.description,
            Some(Description),
        );
        out.newline(indent);

        foreach(
            out,
            &self.options,
            "OPTIONS".to_string(),
            indent,
        );
        foreach(
            out,
            &self.arguments,
            "ARGUMENTS".to_string(),
            indent,
        );
    }

    fn usage_line<W: UsageWriter>(&self, out: &mut W, _indent: usize) {
        out.write(
            &self.name,
            Some(Name),
        );
        out.write(
            " -- ", None,
        );
        out.write_opt(
            &self.description,
            Some(Description),
        );
    }
}

impl Usage for OptionElement {
    fn usage<W: UsageWriter>(&self, out: &mut W, indent: usize) {
        self.usage_line(
            out, indent,
        );
        out.newline(indent);

        foreach(
            out,
            &self.arguments,
            "ARGUMENTS".to_string(),
            indent,
        );
    }

    fn usage_line<W: UsageWriter>(&self, out: &mut W, indent: usize) {
        let joined = self
            .aliases
            .as_ref()
            .map(|v| v.join(", ") + ", ")
            .unwrap_or_default();

        out.write(
            joined + &self.name,
            Some(Name),
        );
        out.newline(indent);
        out.write_opt(
            &self.description,
            Some(Description),
        );
    }
}

impl Usage for ArgumentElement {
    fn usage<W: UsageWriter>(&self, out: &mut W, indent: usize) {
        self.usage_line(
            out, indent,
        );
    }

    fn usage_line<W: UsageWriter>(&self, out: &mut W, indent: usize) {
        let name = if !self.required {
            &format!(
                "[{}]",
                self.name
                    .to_uppercase()
            )
        } else {
            &format!(
                "<{}>",
                self.name
                    .to_uppercase()
            )
        };

        let arity = &format!(
            "{}",
            self.arity
                .as_ref()
                .map(|x| x.to_string())
                .unwrap_or("no arity".to_string())
        );

        out.write(
            name,
            Some(Name),
        );
        out.write(
            " -- ", None,
        );
        out.write(
            arity,
            Some(Details),
        );
        out.newline(indent);

        out.write_opt(
            &self.description,
            Some(Description),
        );
    }
}

impl Usage for ExitCodeElement {
    fn usage<W: UsageWriter>(&self, out: &mut W, indent: usize) {
        self.usage_line(
            out, indent,
        );
    }

    fn usage_line<W: UsageWriter>(&self, out: &mut W, _indent: usize) {
        out.write(
            self.code
                .to_string(),
            Some(Name),
        );
        out.write(
            " -- ", None,
        );
        out.write_opt(
            &self.description,
            Some(Description),
        );
    }
}

impl Usage for String {
    fn usage<W: UsageWriter>(&self, out: &mut W, indent: usize) {
        self.usage_line(
            out, indent,
        );
    }

    fn usage_line<W: UsageWriter>(&self, out: &mut W, _indent: usize) {
        out.write(
            self.as_str(),
            None,
        );
    }
}

impl Display for Arity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "min: {} | max: {}",
            self.minimum
                .map(|m| m.to_string())
                .unwrap_or("N/A".to_string()),
            self.maximum
                .map(|m| m.to_string())
                .unwrap_or("N/A".to_string()),
        );
        write!(
            f,
            "{}",
            str
        )
    }
}
