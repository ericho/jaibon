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
    pub stdout: String,
    pub stderr: String,
    pub node: String,
    user: String,
    pub result: CommandResult,
    pub printer: CommandPrinter,
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
            printer: CommandPrinter::DefaultPrinter,
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

    fn default_formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = match self.result {
            Ok(_) => "ok",
            Err(_) => "error",
        };
        write!(f, "==== Command '{}' in node '{}' ====\n\
                   Status : {}\nStdout : \n{}\nStderr : \n{}\n",
               self.command,
               self.node,
               status,
               self.stdout,
               self.stderr)
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
        let c = Command::new("user".to_owned(),
                             "node",
                             "command".to_owned());
        assert_eq!("==== Command 'command' in node 'node' ====\nStatus : ok\
                   \nStdout : \n\nStderr : \n\n", format!("{}", c));
    }
}
