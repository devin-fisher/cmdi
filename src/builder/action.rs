use crate::builder::Action::*;
use crate::screens::KeyBindingType::Modifier;
use crate::screens::{KeyBinding, KeyBindingType};
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Action {
    ReplaceToken,
    InsertOptionBelow,
    InsertArgument,
    InsertCommand,
    LookupArguments,
    RemoveToken,
}

impl Action {
    pub(crate) fn keybinding_to_action(keybinding: char, actions: Vec<Action>) -> Option<Action> {
        actions
            .iter()
            .find(
                |action| {
                    action
                        .keybinding()
                        .0
                        == keybinding
                },
            )
            .cloned()
    }
}

impl PartialOrd<Self> for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank()
            .cmp(&other.rank())
    }
}

impl KeyBinding for Action {
    fn rank(&self) -> usize {
        match self {
            ReplaceToken => 10,
            InsertOptionBelow => 29,
            InsertArgument => 28,
            InsertCommand => 27,
            LookupArguments => 30,
            RemoveToken => 40,
        }
    }

    fn keybinding(
        &self,
    ) -> (
        char,
        KeyBindingType,
    ) {
        match self {
            ReplaceToken => (
                'e', Modifier,
            ),
            InsertOptionBelow => (
                'o', Modifier,
            ),
            InsertArgument => (
                'a', Modifier,
            ),
            InsertCommand => (
                'c', Modifier,
            ),
            LookupArguments => (
                'l', Modifier,
            ),
            RemoveToken => (
                'r', Modifier,
            ),
        }
    }

    fn display_name(&self) -> String {
        match self {
            ReplaceToken => t!("action_hints.edit").to_string(),
            InsertOptionBelow => t!("action_hints.insert_option").to_string(),
            InsertArgument => t!("action_hints.insert_argument").to_string(),
            InsertCommand => t!("action_hints.insert_command").to_string(),
            LookupArguments => t!("action_hints.lookup_arguments").to_string(),
            RemoveToken => t!("action_hints.remove").to_string(),
        }
    }
}
