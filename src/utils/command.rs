use std::process::Command;

pub fn create_command(command: String) -> Command {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    cmd
}
