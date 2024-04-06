use super::SendArgs;
use crate::utils::stdin_to_string;

pub fn handle_message(args: &SendArgs) -> Option<String> {
    if let Some(message) = &args.message {
        Some(message.clone())
    } else if args.input {
        Some(stdin_to_string().trim_end().to_string())
    } else {
        None
    }
}
