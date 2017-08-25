use clap::{App, Arg};


// The intended usage of this program is
// jb -u --user <user> -w <list_of_hostnames> -c <Command> --quiet
pub fn create_cli() -> App<'static, 'static> {

    let user = Arg::with_name("user")
        .short("u")
        .long("user")
        .takes_value(true)
        .help("Set the username used to perform the SSH connection \
               to the specified hosts. If is not set, the user will \
               be retrieved from the $USER environmental variable.");

    let nodes = Arg::with_name("nodes")
        .short("n")
        .long("nodes")
        .takes_value(true)
        .required(true)
        .help("Specify the node or list of nodes where the command \
               will be executed. The list of nodes should be set in a \
               separated comma list format.");

    let command = Arg::with_name("command")
        .short("c")
        .long("command")
        .takes_value(true)
        .required(true)
        .help("The command to be executed in the remote hosts.");

    let background = Arg::with_name("background")
        .short("b")
        .long("background")
        .takes_value(false)
        .required(false)
        .help("Use this flag when a command is intended to be \
               executed and keep running in the remote system. \
               for instance, commands like `someprogram &`. This \
               flag requires that `nohup` is present in the \
               remote system.");
    // Another commands considered for a future implementation
    // quiet : Just prints the error output
    // json : Print the output in JSON format
    App::new("jaibon")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(user)
        .arg(nodes)
        .arg(command)
        .arg(background)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_created_cli() {
        let cli = create_cli();
        assert_eq!(cli.get_name(), "jaibon");
        let args = vec!["jaibon", "-u", "user", "-c", "mycmd", "-n", "nodes", "-b"];
        let matches = cli.get_matches_from(args);
        assert_eq!(matches.value_of("user"), Some("user"));
        assert_eq!(matches.value_of("command"), Some("mycmd"));
        assert_eq!(matches.value_of("nodes"), Some("nodes"));
        assert_eq!(matches.is_present("background"), true);
    }
}
