// use crate::opencli::v0_1::{OptionElement};

// pub fn find_option_spec<'a>(
//     options: &'a Vec<OptionElement>,
//     option_flag: &str,
// ) -> Option<&'a OptionElement> {
//     options
//     .iter()
//         .find(|o| o.name == option_flag)
// }

// pub fn find_options(spec: &V0_1, cmd_ctx: Option<&CommandElement>) -> Vec<String> {
//     cmd_ctx
//         .and_then(|cmd| cmd.options.as_ref())
//         .or_else(|| spec.options.as_ref())
//         .map(|elems| elems.iter().map(|s| s.name.clone()).collect())
//         .unwrap_or_default()
// }
