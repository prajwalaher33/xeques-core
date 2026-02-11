use crate::protocol::command::Command;

#[derive(Clone)]
pub struct QueuedCommand {
    pub command: Command,
    pub received_at: u64,
    pub role: String,
}
