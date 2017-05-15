use std;
use std::marker::PhantomData;

pub enum CommandErrors {
    RuntimeError
}

pub type CommandResult = Result<(), CommandErrors>;

pub struct Command {
    command: String,
    pub stdout: String,
    pub stderr: String,
    node: String,
    user: String
}

impl Command {
    pub fn new(user: &str, node: &str, command: &str) -> Command {
        Command {
            command: command.to_string(),
            user: user.to_string(),
            node: node.to_string(),
            stdout: String::new(),
            stderr: String::new()
        }
    }

    pub fn run(&mut self) -> CommandResult {
        let cmd = self.create_scp_command();
        let s_cmd: Vec<&str> = cmd.split(' ').collect();
        let output = std::process::Command::new(&s_cmd[0])
            .args(s_cmd.split_at(1).1)
            .output()
            .expect("Failure running command");

        let tmp = std::str::from_utf8(&output.stdout).unwrap();
        self.stdout.push_str(tmp);

        let tmp = std::str::from_utf8(&output.stderr).unwrap();
        self.stderr.push_str(tmp);

        if output.status.success() {
            Ok(())
        } else {
            Err(CommandErrors::RuntimeError)
        }
    }

    fn create_scp_command(&self) -> String {
        let s = format!("ssh {}@{} {}", self.user, self.node, self.command);
        s
    }
}
