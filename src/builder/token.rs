use crate::builder::token::Token::*;
use crate::opencli::v0_1::{ArgumentElement, CommandElement, OptionElement, V0_1};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Token {
    PlaceholderToken,
    CommandToken {
        ctx: V0_1,
    },
    OptionToken {
        ctx: Option<CommandContext>,
        spec: OptionElement,
        arg: Vec<String>,
        details: Vec<Detail>,
    },
    SubCommandToken {
        ctx: CommandContext,
        details: Vec<Detail>,
    },
    ArgumentToken {
        ctx: Option<CommandContext>,
        spec: ArgumentElement,
        arg: String,
        details: Vec<Detail>,
    },
}

#[derive(Clone, Debug)]
pub enum Detail {}

#[derive(Clone, Debug)]
pub struct CommandContext {
    pub level: usize,
    pub(crate) spec: CommandElement,
}

impl CommandContext {
    pub(crate) fn new(level: usize, spec: CommandElement) -> Self {
        Self {
            level,
            spec,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceholderToken => write!(
                f,
                "PlaceholderToken"
            ),
            CommandToken {
                ..
            } => write!(
                f,
                "CommandToken"
            ),
            OptionToken {
                ..
            } => write!(
                f,
                "OptionToken"
            ),
            SubCommandToken {
                ..
            } => write!(
                f,
                "SubCommandToken"
            ),
            ArgumentToken {
                ..
            } => write!(
                f,
                "ArgumentToken"
            ),
        }
    }
}

impl Token {
    pub(crate) fn args(&self) -> Vec<String> {
        match self {
            PlaceholderToken => vec!(),
            CommandToken { .. } => vec!(),
            OptionToken { arg, ..} => arg.to_owned(),
            SubCommandToken { .. } => vec!(),
            ArgumentToken {arg, .. } =>  vec!(arg.to_owned())
        }
    }
    pub(crate) fn level(&self) -> usize {
        match self {
            CommandToken {
                ..
            } => 0,
            SubCommandToken {
                ctx,
                ..
            } => ctx.level,
            ArgumentToken {
                ctx,
                ..
            } => Self::opt_to_level(ctx),
            PlaceholderToken => 0,
            OptionToken {
                ctx,
                ..
            } => Self::opt_to_level(ctx),
        }
    }

    fn opt_to_level(ctx: &Option<CommandContext>) -> usize {
        ctx.as_ref()
            .map(|x| x.level)
            .unwrap_or(1)
    }
}
