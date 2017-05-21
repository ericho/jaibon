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

    // Another commands considered for a future implementation
    // quiet : Just prints the error output
    // json : Print the output in JSON format
    App::new("jaibon")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(user)
        .arg(nodes)
        .arg(command)
}
