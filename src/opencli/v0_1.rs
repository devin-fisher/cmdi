// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::V0_1;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: V0_1 = serde_json::from_str(&json).unwrap();
// }
// Generated via quicktype (https://app.quicktype.io/) using
// opencli 0.1 spec (https://github.com/spectreconsole/open-cli/blob/df8233e382e9013b7e4e9b231875a5a3ffb91397/schema.json)
//
// Added default values by hand

use serde::{Deserialize, Serialize};

fn default_false() -> bool {
    false
}

/// The OpenCLI description
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V0_1 {
    /// Root command arguments
    pub arguments: Option<Vec<ArgumentElement>>,

    /// Root command sub commands
    pub commands: Option<Vec<CommandElement>>,

    /// The conventions used by the CLI
    pub conventions: Option<Conventions>,

    /// Examples of how to use the CLI
    pub examples: Option<Vec<String>>,

    /// Root command exit codes
    pub exit_codes: Option<Vec<ExitCodeElement>>,

    /// Information about the CLI
    pub info: Info,

    /// Indicates whether or not the command requires interactive input
    #[serde(default = "default_false")]
    pub interactive: bool,

    /// Custom metadata
    pub metadata: Option<Vec<MetadatumElement>>,

    /// The OpenCLI version number
    pub opencli: String,

    /// Root command options
    pub options: Option<Vec<OptionElement>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArgumentElement {
    /// A list of accepted values
    pub accepted_values: Option<Vec<String>>,

    /// The argument arity. Arity defines the minimum and maximum number of argument values
    pub arity: Option<Arity>,

    /// The argument description
    pub description: Option<String>,

    /// The argument group
    pub group: Option<String>,

    /// Whether or not the argument is hidden
    #[serde(default = "default_false")]
    pub hidden: bool,

    /// Custom metadata
    pub metadata: Option<Vec<MetadatumElement>>,

    /// The argument name
    pub name: String,

    /// Whether or not the argument is required
    #[serde(default = "default_false")]
    pub required: bool,
}

/// The argument arity. Arity defines the minimum and maximum number of argument values
///
/// Arity defines the minimum and maximum number of argument values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arity {
    /// The maximum number of values allowed
    pub maximum: Option<i64>,

    /// The minimum number of values allowed
    pub minimum: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadatumElement {
    pub name: String,

    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandElement {
    /// The command aliases
    pub aliases: Option<Vec<String>>,

    /// The command arguments
    pub arguments: Option<Vec<ArgumentElement>>,

    /// The command's sub commands
    pub commands: Option<Vec<CommandElement>>,

    /// The command description
    pub description: Option<String>,

    /// Examples of how to use the command
    pub examples: Option<Vec<String>>,

    /// The command's exit codes
    pub exit_codes: Option<Vec<ExitCodeElement>>,

    /// Whether or not the command is hidden
    #[serde(default = "default_false")]
    pub hidden: bool,

    /// Indicate whether or not the command requires interactive input
    #[serde(default = "default_false")]
    pub interactive: bool,

    /// Custom metadata
    pub metadata: Option<Vec<MetadatumElement>>,

    /// The command name
    pub name: String,

    /// The command options
    pub options: Option<Vec<OptionElement>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExitCodeElement {
    /// The exit code
    pub code: i64,

    /// The exit code description
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionElement {
    /// The option's aliases
    pub aliases: Option<Vec<String>>,

    /// The option's arguments
    pub arguments: Option<Vec<ArgumentElement>>,

    /// The option description
    pub description: Option<String>,

    /// The option group
    pub group: Option<String>,

    /// Whether or not the option is hidden
    #[serde(default = "default_false")]
    pub hidden: bool,

    /// Custom metadata
    pub metadata: Option<Vec<MetadatumElement>>,

    /// The option name
    pub name: String,

    /// Specifies whether the option is accessible from the immediate parent command and,
    /// recursively, from its subcommands
    #[serde(default = "default_false")]
    pub recursive: bool,

    /// Whether or not the option is required
    #[serde(default = "default_false")]
    pub required: bool,
}

/// The conventions used by the CLI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conventions {
    /// Whether or not grouping of short options are allowed
    pub group_options: Option<bool>,

    /// The option argument separator
    pub option_separator: Option<String>,
}

/// Information about the CLI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    /// The contact information
    pub contact: Option<Contact>,

    /// A description of the application
    pub description: Option<String>,

    /// The application license
    pub license: Option<License>,

    /// A short summary of the application
    pub summary: Option<String>,

    /// The application title
    pub title: String,

    /// The application version
    pub version: String,
}

/// The contact information
///
/// Contact information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    /// The email address of the contact person/organization. This MUST be in the form of an
    /// email address.
    pub email: Option<String>,

    /// The identifying name of the contact person/organization
    pub name: Option<String>,

    /// The URI for the contact information. This MUST be in the form of a URI.
    pub url: Option<String>,
}

/// The application license
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct License {
    /// The SPDX license identifier
    pub identifier: Option<String>,

    /// The license name
    pub name: Option<String>,
}
