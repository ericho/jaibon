use std;
use std::fmt;

pub enum CommandErrors {
    RuntimeError,
}

pub enum CommandPrinter {
    DefaultPrinter,
}

pub type CommandResult = Result<(), CommandErrors>;

pub struct Command {
    command: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub node: String,
    user: String,
    pub result: CommandResult,
    pub printer: CommandPrinter,
}

impl Command {
    pub fn new(user: &str, node: &str, command: &str) -> Command {
        Command {
            command: command.to_owned(),
            user: user.to_owned(),
            node: node.to_owned(),
            stdout: None,
            stderr: None,
            result: Ok(()),
            printer: CommandPrinter::DefaultPrinter,
        }
    }

    pub fn run(&mut self) {
        let cmd = self.create_ssh_command();
        let s_cmd: Vec<&str> = cmd.split(' ').collect();
        let output = std::process::Command::new(&s_cmd[0])
            .args(s_cmd.split_at(1).1)
            .output()
            .expect("Failure running command");

        let tmp = std::str::from_utf8(&output.stdout)
            .unwrap().to_owned();
        self.stdout = Some(tmp);

        let tmp = std::str::from_utf8(&output.stderr)
            .unwrap().to_owned();
        self.stderr = Some(tmp);

        if output.status.success() {
            self.result = Ok(());
        } else {
            self.result = Err(CommandErrors::RuntimeError);
        }
    }

    fn create_ssh_command(&self) -> String {
        let s = format!("ssh {}@{} {}", self.user, self.node, self.command);
        s
    }

    fn default_formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match self.result {
            Ok(_) => "ok",
            Err(_) => "error",
        };

        let default = String::from("empty");
        let stdout = self.stdout.as_ref().unwrap_or(&default);
        let stderr = self.stderr.as_ref().unwrap_or(&default);
        write!(f,
               "==== Command '{}' in node '{}' ====\n\
                   Status : {}\nStdout : \n{}\nStderr : \n{}\n",
               self.command,
               self.node,
               status,
               stdout,
               stderr)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.printer {
            CommandPrinter::DefaultPrinter => self.default_formatter(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_formatter() {
        let c = Command::new("user", "node", "command");
        assert_eq!("==== Command 'command' in node 'node' ====\nStatus : ok\
                   \nStdout : \nempty\nStderr : \nempty\n",
                   format!("{}", c));
    }

    #[test]
    fn run_basic_command() {
        let mut c = Command::new("user", "node", "command");
        c.run();
        assert_eq!(c.result.is_err(), true);
    }
}
