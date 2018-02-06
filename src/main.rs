#![cfg_attr(feature = "clippy", allow(unstable_features))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate threadpool;
extern crate num_cpus;

mod commands;

use std::env;
use std::sync::mpsc::channel;

use threadpool::ThreadPool;
use commands::Command;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jb", about = "An utility to execute commands in remote hosts through ssh.")]
struct Opt {
    #[structopt(short = "u",
                long = "user",
                help = "Set the username used to perform the SSH connection \
                        to the specified hosts. If is not set, the user will \
                        be retrieved from the $USER environmental variable.")]
    user: Option<String>,

    #[structopt(short = "n",
                long = "nodes",
                help = "Specify the node or list of nodes where the command \
                        will be executed. The list of nodes should be set in a \
                        separated comma list format.")]
    nodes: String,

    #[structopt(short = "c",
                long = "command",
                help = "The command to be executed in the remote hosts.")]
    command: String,

    #[structopt(short = "b",
                long = "background",
                help = "Use this flag when a command is intended to be \
                        executed and keep running in the remote system. \
                        for instance, commands like `someprogram &`. This \
                        flag requires that `nohup` is present in the \
                        remote system.")]
    background: bool,
}

fn create_nodes_vector(nodes: &str) -> Vec<String> {
    nodes.split_terminator(',')
        .map(|x| x.trim().to_string())
        .collect()
}

fn main() {
    let opts = Opt::from_args();

    let user = opts.user.unwrap_or(env::var("USER").unwrap().to_string());

    let nodes = opts.nodes;
    let command = opts.command;
    let background = opts.background;
    let nodes_vec = create_nodes_vector(&nodes);

    let num_cpus = num_cpus::get();

    let (tx, rx) = channel();
    let pool = ThreadPool::new(num_cpus);

    for node in &nodes_vec {
        let user = user.clone();
        let command = command.clone();
        let tx = tx.clone();
        let node = node.clone();
        pool.execute(move || {
            println!("Launching command on node {}", node);
            let mut cmd =
                Command::new(&user,
                             &node,
                             &command,
                             background);
            cmd.run();
            tx.send(cmd).unwrap();
        });
    }

    for t in rx.iter().take(nodes_vec.len()) {
        println!("{}", t);
    }
    println!("Done");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_nodes_vector_with_one_nodes() {
        let r = create_nodes_vector("node1");
        assert_eq!(1, r.len());
        assert_eq!(r, ["node1"]);
    }

    #[test]
    fn create_nodes_vector_with_two_nodes() {
        let r = create_nodes_vector("node1,node2");
        assert_eq!(2, r.len());
        assert_eq!(r, ["node1", "node2"]);
    }

    #[test]
    fn create_nodes_vector_with_space_in_nodes() {
        let r = create_nodes_vector("node1, node2");
        assert_eq!(2, r.len());
        assert_eq!(r, ["node1", "node2"]);
    }

    #[test]
    fn create_nodes_vector_with_space_in_three_nodes() {
        let r = create_nodes_vector("node1, node2,node3");
        assert_eq!(3, r.len());
        assert_eq!(r, ["node1", "node2", "node3"]);
    }

    #[test]
    fn create_nodes_vector_with_lot_of_spaces() {
        let r = create_nodes_vector("node1,      node2,    node3     ,node4");
        assert_eq!(4, r.len());
        assert_eq!(r, ["node1", "node2", "node3", "node4"]);
    }

    #[test]
    fn create_nodes_vector_trailing_comma() {
        let r = create_nodes_vector("node1,");
        assert_eq!(1, r.len());
        assert_eq!(r, ["node1"]);
    }

    #[test]
    fn create_nodes_vector_trailing_2_comma() {
        let r = create_nodes_vector("node1,node2,");
        assert_eq!(2, r.len());
        assert_eq!(r, ["node1", "node2"]);
    }
}
