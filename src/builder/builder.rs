use crate::builder::Token::{
    ArgumentToken, CommandToken, OptionToken, PlaceholderToken, SubCommandToken,
};
use crate::builder::{Action, CommandContext, Token};
use crate::opencli::v0_1::V0_1;
use crate::util::util::try_insert;
use color_eyre::eyre::bail;
use std::cmp::min;

#[derive()]
pub struct Builder {
    pub command_spec: V0_1,
    command_tokens: Vec<Token>,
    command_selected_pos: usize,
}

impl Builder {
    pub(crate) fn do_token_action(
        &mut self,
        token: Token,
        action: Action,
    ) -> color_eyre::Result<()> {
        match action {
            Action::ReplaceToken => Ok(()),
            Action::InsertOptionBelow => self.insert_below_selected(token),
            Action::InsertArgument => {
                self.replace_at_selected(token)?;
                Ok(())
            },
            Action::InsertCommand => Ok(()),
            Action::LookupArguments => Ok(()),
            Action::RemoveToken => Ok(()),
        }
    }
}

// Static functions
impl Builder {}

// Constructors
impl Builder {
    pub fn new(command_spec: V0_1) -> Self {
        Self {
            command_spec,
            command_tokens: vec![PlaceholderToken],
            command_selected_pos: 0,
        }
    }

    pub fn new_demo(command_spec: V0_1, command_tokens: Vec<Token>) -> Self {
        let mut rtn = Self {
            command_spec,
            command_tokens,
            command_selected_pos: 0,
        };
        rtn.condition_tokens();
        rtn
    }
}

// Methods
impl Builder {
    pub fn cmd_title(&self) -> &String {
        &self
            .command_spec
            .info
            .title
    }

    pub fn tokens(&self) -> &[Token] {
        &self.command_tokens
    }

    fn condition_tokens(&mut self) {
        if let Some(PlaceholderToken) = self
            .command_tokens
            .last()
        {
            // No action
        } else {
            self.command_tokens
                .push(PlaceholderToken);
        }
    }

    pub fn available_actions(&self) -> Vec<Action> {
        match self.token_at_selected() {
            None => vec![],
            Some(PlaceholderToken) => vec![],
            Some(CommandToken {
                ..
            }) => vec![Action::InsertOptionBelow],
            Some(OptionToken {
                spec,
                ..
            }) => {
                if spec
                    .arguments
                    .is_some()
                {
                    vec![Action::InsertArgument]
                } else {
                    vec![]
                }
            }
            Some(SubCommandToken {
                ..
            }) => vec![Action::InsertOptionBelow],
            Some(ArgumentToken {
                ..
            }) => vec![],
        }
    }

    #[allow(dead_code)]
    fn command_context_at_selected(&self) -> Option<&CommandContext> {
        self.command_tokens
            .iter()
            .enumerate()
            .take(self.command_selected_pos + 1)
            .rev()
            .find_map(
                |(_i, token)| {
                    if let SubCommandToken {
                        ctx: context,
                        ..
                    } = token
                    {
                        Some(context)
                    } else {
                        None
                    }
                },
            )
    }

    pub fn pos_at_selected(&self) -> usize {
        self.command_selected_pos
    }

    pub fn token_at_selected(&self) -> Option<&Token> {
        self.command_tokens
            .get(self.command_selected_pos)
    }

    pub(crate) fn replace_at_selected(&mut self, token: Token) -> color_eyre::Result<()> {
        let i = self.command_selected_pos;
        if let Some(slot) = self
            .command_tokens
            .get_mut(i)
        {
            *slot = token;
            self.condition_tokens();
            Ok(())
        } else {
            bail!(
                "Index {} out of bounds (len = {})",
                i,
                self.command_tokens
                    .len()
            )
        }
    }
    pub(crate) fn insert_below_selected(&mut self, token: Token) -> color_eyre::Result<()> {
        match self
            .command_tokens
            .get(self.command_selected_pos)
        {
            // replace the placeholder instead of inserting below
            Some(PlaceholderToken) => self.replace_at_selected(token),
            Some(_) => {
                let insertion_pos = self.command_selected_pos + 1;
                try_insert(
                    &mut self.command_tokens,
                    insertion_pos,
                    token,
                )?;
                Ok(())
            }
            None => bail!(
                "Index {} out of bounds",
                self.command_selected_pos
            ),
        }
    }

    pub(crate) fn selected_up(&mut self) {
        self.command_selected_pos = self
            .command_selected_pos
            .saturating_sub(1);
    }
    pub(crate) fn selected_down(&mut self) {
        if self
            .command_tokens
            .is_empty()
        {
            return;
        }

        // Make sure we don't move beyond the size of the token vec
        self.command_selected_pos = min(
            self.command_selected_pos
                .saturating_add(1),
            self.command_tokens
                .len()
                - 1,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::Token::{PlaceholderToken, SubCommandToken};
    use crate::opencli::v0_1::{CommandElement, V0_1};
    use color_eyre::eyre::Result;

    fn sample_command_element() -> CommandElement {
        sample_v0_1()
            .commands
            .unwrap()
            .first()
            .unwrap()
            .clone()
    }

    fn sample_v0_1() -> V0_1 {
        serde_yml::from_str(
            r#"
---
"$schema": https://json-schema.org/draft/2020-12/schema
"$id": OpenCLI_kubectl.json
opencli: '0.1'
info:
  title: kubectl
  version: 1.29.0
  summary: Command-line tool for controlling Kubernetes clusters
  description: kubectl controls Kubernetes clusters. You can deploy applications,
    inspect and manage cluster resources, and view logs.
  contact:
    name: Kubernetes Contributors
    url: https://kubernetes.io/docs/reference/kubectl/
    email: kubernetes-dev@googlegroups.com
  license:
    name: Apache License 2.0
    identifier: Apache-2.0
conventions:
  groupOptions: true
  optionSeparator: " "
options:
  - name: "--kubeconfig"
    description: Path to the kubeconfig file to use
  - name: "--context"
    description: The name of the kubeconfig context to use
commands:
  - name: get
    description: Display one or many resources
    arguments:
      - name: resource
        required: true
      - name: name
        required: false
    options:
      - name: "--namespace"
        description: If present, the namespace scope for this CLI request
      - name: "-o"
        description: 'Output format. One of: json|yaml|wide'
  - name: apply
    description: Apply a configuration to a resource by filename or stdin
    options:
      - name: "-f"
        description: Filename or directory to apply
        required: true
  - name: delete
    description: Delete resources by filenames, stdin, resources and names, or by label
      selector
    arguments:
      - name: resource
        required: true
      - name: name
        required: false
examples:
  - kubectl get pods
  - kubectl apply -f deployment.yaml
  - kubectl delete pod nginx
interactive: false
        "#,
        )
        .unwrap()
    }

    #[test]
    fn test_new_builder() {
        let spec = sample_v0_1();
        let builder = Builder::new(spec.clone());
        assert_eq!(
            builder
                .command_tokens
                .len(),
            1
        );
        assert!(
            matches!(
                builder.command_tokens[0],
                PlaceholderToken
            )
        );
        assert_eq!(
            builder.command_selected_pos,
            0
        );
        assert_eq!(
            builder
                .command_spec
                .info
                .title,
            spec.info
                .title
        );
    }

    #[test]
    fn test_new_demo_builder_adds_placeholder() {
        let spec = sample_v0_1();
        let tokens = vec![
            SubCommandToken {
                ctx: CommandContext::new(
                    0,
                    sample_command_element(),
                ),
                details: vec![],
            },
        ];
        let builder = Builder::new_demo(
            spec,
            tokens.clone(),
        );

        // Ensure placeholder added
        assert_eq!(
            builder
                .command_tokens
                .len(),
            tokens.len() + 1
        );
        assert!(
            matches!(
                builder
                    .command_tokens
                    .last()
                    .unwrap(),
                PlaceholderToken
            )
        );
    }

    #[test]
    fn test_condition_tokens_no_action_if_last_is_placeholder() {
        let spec = sample_v0_1();
        let mut builder = Builder::new(spec);
        builder.condition_tokens();
        // Should still have only 1 placeholder
        assert_eq!(
            builder
                .command_tokens
                .len(),
            1
        );
        assert!(
            matches!(
                builder.command_tokens[0],
                PlaceholderToken
            )
        );
    }

    #[test]
    fn test_command_context_at_selected_returns_last_command_token() {
        let spec = sample_v0_1();
        let tokens = vec![
            SubCommandToken {
                ctx: CommandContext::new(
                    0,
                    sample_command_element(),
                ),
                details: vec![],
            },
            PlaceholderToken,
        ];
        let mut builder = Builder::new_demo(
            spec, tokens,
        );
        builder.command_selected_pos = 1;

        let context = builder.command_context_at_selected();
        assert!(context.is_some());
        assert_eq!(
            context
                .unwrap()
                .spec
                .name,
            "get"
        );
    }

    #[test]
    fn test_token_at_selected_returns_correct_token() {
        use crate::builder::Token::{PlaceholderToken, SubCommandToken};

        let spec = sample_v0_1();

        // --- Case 1: Single PlaceholderToken ---
        let builder = Builder::new_demo(
            spec.clone(),
            vec![PlaceholderToken],
        );
        let token = builder.token_at_selected();
        assert!(
            matches!(
                token.unwrap(),
                PlaceholderToken
            )
        );

        // --- Case 2: Multiple tokens, selected at 0 ---
        let cmd1 = SubCommandToken {
            ctx: CommandContext::new(
                0,
                sample_command_element(),
            ),
            details: vec![],
        };
        let builder = Builder::new_demo(
            spec.clone(),
            vec![
                cmd1.clone(),
                PlaceholderToken,
            ],
        );
        let token = builder.token_at_selected();
        assert!(
            matches!(
                token.unwrap(),
                SubCommandToken { .. }
            )
        );

        // --- Case 3: Selected in the middle ---
        let mut builder = Builder::new_demo(
            spec.clone(),
            vec![
                cmd1.clone(),
                PlaceholderToken,
                cmd1.clone(),
            ],
        );
        builder.command_selected_pos = 1;
        let token = builder.token_at_selected();
        assert!(
            matches!(
                token.unwrap(),
                PlaceholderToken
            )
        );

        builder.command_selected_pos = 2;
        let token = builder.token_at_selected();
        assert!(
            matches!(
                token.unwrap(),
                SubCommandToken { .. }
            )
        );

        // --- Case 4: Selected position out of bounds ---
        let mut builder = Builder::new_demo(
            spec.clone(),
            vec![cmd1.clone()],
        );
        builder.command_selected_pos = 5;
        let token = builder.token_at_selected();
        assert!(token.is_none());

        // --- Case 5: Empty command_tokens vector returns placeholder ---
        let mut builder = Builder::new(spec.clone());
        builder.command_selected_pos = 0;
        let token = builder.token_at_selected();
        assert!(
            matches!(
                token.unwrap(),
                PlaceholderToken
            )
        );
    }

    #[test]
    fn test_replace_at_selected_replaces_and_adds_placeholder() -> Result<()> {
        let spec = sample_v0_1();
        let tokens = vec![PlaceholderToken];
        let mut builder = Builder::new_demo(
            spec, tokens,
        );

        let new_token = SubCommandToken {
            ctx: CommandContext::new(
                0,
                sample_command_element(),
            ),
            details: vec![],
        };
        builder.replace_at_selected(new_token.clone())?;

        assert!(
            matches!(
                builder.command_tokens[0],
                SubCommandToken { .. }
            )
        );
        assert!(
            matches!(
                builder.command_tokens[1],
                PlaceholderToken
            )
        );
        Ok(())
    }

    #[test]
    fn test_replace_at_selected_out_of_bounds() {
        let spec = sample_v0_1();
        let mut builder = Builder::new(spec);

        builder.command_selected_pos = 5; // intentionally out of bounds
        let new_token = SubCommandToken {
            ctx: CommandContext::new(
                0,
                sample_command_element(),
            ),
            details: vec![],
        };
        let result = builder.replace_at_selected(new_token);

        assert!(result.is_err());
    }

    #[test]
    fn test_insert_below_selected_replaces_placeholder() -> Result<()> {
        let spec = sample_v0_1();
        let mut builder = Builder::new(spec);

        let new_token = SubCommandToken {
            ctx: CommandContext::new(
                0,
                sample_command_element(),
            ),
            details: vec![],
        };
        builder.insert_below_selected(new_token.clone())?;

        assert!(
            matches!(
                builder.command_tokens[0],
                SubCommandToken { .. }
            )
        );
        assert!(
            matches!(
                builder.command_tokens[1],
                PlaceholderToken
            )
        );
        Ok(())
    }

    #[test]
    fn test_insert_below_selected_inserts_after_non_placeholder() -> Result<()> {
        let spec = sample_v0_1();
        let cmd_token = SubCommandToken {
            ctx: CommandContext::new(
                0,
                sample_command_element(),
            ),
            details: vec![],
        };
        let mut builder = Builder::new_demo(
            spec,
            vec![
                cmd_token.clone(),
                PlaceholderToken,
            ],
        );

        builder.command_selected_pos = 0;
        let new_token = SubCommandToken {
            ctx: CommandContext::new(
                0,
                sample_command_element(),
            ),
            details: vec![],
        };
        builder.insert_below_selected(new_token.clone())?;

        assert_eq!(
            builder
                .command_tokens
                .len(),
            3
        );
        assert!(
            matches!(
                builder.command_tokens[1],
                SubCommandToken { .. }
            )
        );
        Ok(())
    }

    #[test]
    fn test_selected_up_down() {
        let spec = sample_v0_1();
        let mut builder = Builder::new(spec);

        builder.selected_down();
        assert_eq!(
            builder.command_selected_pos,
            0
        );

        builder.selected_up();
        assert_eq!(
            builder.command_selected_pos,
            0
        );

        builder.selected_up(); // saturating_sub prevents underflow
        assert_eq!(
            builder.command_selected_pos,
            0
        );
    }

    #[test]
    fn test_selected_down_does_not_exceed_len() {
        let spec = sample_v0_1();
        let mut builder = Builder::new(spec);
        builder.command_selected_pos = 100;

        builder.selected_down();
        assert_eq!(
            builder.command_selected_pos,
            builder
                .command_tokens
                .len()
                - 1
        );
    }

    #[test]
    fn test_select_down_can_not_move_beyond_limit() {
        let spec = sample_v0_1();
        let mut builder = Builder::new(spec);
        builder.selected_down();

        assert_eq!(
            builder.command_selected_pos,
            0
        );
    }
}
