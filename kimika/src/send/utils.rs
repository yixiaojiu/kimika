use super::SendArgs;
use crate::utils::crossterm::try_read_from_pipeline;

pub fn handle_message(args: &SendArgs) -> Option<String> {
    if let Some(message) = &args.message {
        Some(message.clone())
    } else {
        try_read_from_pipeline()
    }
}
