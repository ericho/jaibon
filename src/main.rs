#![cfg_attr(feature = "clippy", allow(unstable_features))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clap;
extern crate threadpool;
extern crate num_cpus;

mod cli;
mod commands;

use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use commands::Command;


fn create_nodes_vector(nodes: &str) -> Vec<String> {
    let v: Vec<&str> = nodes.split_terminator(',').map(|x| x.trim()).collect();
    let mut nodes_vec: Vec<String> = Vec::with_capacity(v.len());
    for node in v {
        nodes_vec.push(node.to_string());
    }
    nodes_vec
}

fn main() {
    let cli = cli::create_cli().get_matches();

    let user = cli.value_of("user")
        .unwrap_or(env::var("USER").unwrap().as_str())
        .to_owned();
    let nodes = cli.value_of("nodes").unwrap();
    let command = cli.value_of("command").unwrap().to_owned();
    let background = cli.is_present("background");
    let nodes_vec = create_nodes_vector(nodes);

    let num_cpus = num_cpus::get();

    let (tx, rx) = channel();
    let pool = ThreadPool::new(num_cpus);

    for i in &nodes_vec {
        let user = user.clone();
        let command = command.clone();
        let tx = tx.clone();
        let i = i.clone();
        pool.execute(move || {
                         println!("Launching command on node {}", i);
                         let mut cmd =
                         Command::new(&user,
                                      &i,
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
