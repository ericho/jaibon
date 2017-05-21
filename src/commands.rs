use std;

pub enum CommandErrors {
    RuntimeError,
}

pub type CommandResult = Result<(), CommandErrors>;

pub struct Command {
    command: String,
    pub stdout: String,
    pub stderr: String,
    pub node: String,
    user: String,
    pub result: CommandResult,
}

impl Command {
    pub fn new(user: String, node: &str, command: String) -> Command {
        Command {
            command: command,
            user: user,
            node: node.to_string(),
            stdout: String::new(),
            stderr: String::new(),
            result: Ok(()),
        }
    }

    pub fn run(&mut self) {
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
            self.result = Ok(());
        } else {
            self.result = Err(CommandErrors::RuntimeError);
        }
    }

    fn create_scp_command(&self) -> String {
        let s = format!("ssh {}@{} {}", self.user, self.node, self.command);
        s
    }
}
